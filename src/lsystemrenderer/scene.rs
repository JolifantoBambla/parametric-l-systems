use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::{vertex::Vertex, Mesh};
use crate::framework::renderer::drawable::GpuMesh;
use crate::framework::scene::camera::{CameraView, Projection};
use crate::framework::scene::light::LightSource;
use crate::framework::scene::transform::Transform;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::instancing::{Instance, ModelTransform};
use crate::lsystemrenderer::l_system_manager::{turtle::MaterialState, LSystemManager};
use crate::lsystemrenderer::renderer::{
    LightSourcesBindGroup, LightSourcesBindGroupBuilder, RenderObject, RenderObjectBuilder,
};
use crate::lsystemrenderer::scene_descriptor::{
    LSystemSceneDescriptor, SceneObjectDescriptor, SceneResource,
};
use glam::Vec3;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::BufferUsages;
use crate::lsystemrenderer::l_system_manager::turtle::LSystemPrimitive;

struct MeshResource {
    mesh: Arc<GpuMesh>,
    transform: Transform,
}

enum Resource {
    Mesh(MeshResource),
}

impl Resource {
    pub fn mesh(&self) -> &Arc<GpuMesh> {
        match self {
            Resource::Mesh(m) => &m.mesh,
        }
    }
    pub fn get_or_create_mesh(&mut self, _iteration: &usize) -> &Arc<GpuMesh> {
        match self {
            Resource::Mesh(m) => &m.mesh,
        }
    }
    pub fn transform(&self) -> &Transform {
        match self {
            Resource::Mesh(m) => &m.transform,
        }
    }
}

struct SceneMesh {
    mesh: Arc<GpuMesh>,
    instance_buffer: Buffer<Instance>,
    render_objects: Option<Vec<RenderObject>>,
}

struct LSystemObject {
    system: String,
    instance: String,
    target_iteration: u32,
    active_iteration: Option<u32>,
    render_objects: HashMap<u32, Vec<RenderObject>>,
}

enum Primitive {
    LSystem(LSystemObject),
    Mesh(SceneMesh),
}

struct SceneObject {
    transform_buffer: Buffer<ModelTransform>,
    primitive: Primitive,
}

pub struct LSystemScene {
    background_color: Vec3,
    camera: OrbitCamera,
    aspect_ratio: f32,
    ambient_light: LightSource,
    light_sources: Vec<LightSource>,
    light_sources_bind_group: Option<LightSourcesBindGroup>,
    objects: HashMap<String, SceneObject>,
    cylinder_mesh: Arc<GpuMesh>,
    resources: HashMap<String, Resource>,
    l_systems: HashMap<String, HashMap<String, LSystemManager>>,
}

impl LSystemScene {
    pub fn new(
        mut l_systems: HashMap<String, HashMap<String, LSystem>>,
        scene_descriptor: &LSystemSceneDescriptor,
        view_aspect_ratio: f32,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let camera = OrbitCamera::new(
            Projection::new_perspective(f32::to_radians(45.), view_aspect_ratio, 0.0001, 1000.0),
            CameraView::from(scene_descriptor.scene().camera()),
            10.0,
            0.1,
        );

        let ambient_light = if let Some(descriptor) = scene_descriptor.scene().lights().ambient() {
            LightSource::new_ambient(descriptor.color())
        } else {
            LightSource::new_ambient(Vec3::ZERO)
        };
        let mut light_sources = Vec::new();
        for descriptor in scene_descriptor.scene().lights().directional_lights() {
            light_sources.push(LightSource::new_directional(
                descriptor.direction(),
                descriptor.color(),
                descriptor.intensity(),
            ));
        }
        for descriptor in scene_descriptor.scene().lights().point_lights() {
            light_sources.push(LightSource::new_point(
                descriptor.position(),
                descriptor.color(),
                descriptor.intensity(),
            ));
        }

        let l_system_cylinder_mesh = Arc::new(GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(true),
            gpu.device(),
        ));

        let mut resources: HashMap<String, Resource> = HashMap::new();
        if let Some(scene_resources) = scene_descriptor.resources() {
            for (resource_id, resource) in scene_resources.iter() {
                match resource {
                    SceneResource::Obj(descriptor) => {
                        match Mesh::from_obj_source(descriptor.source()) {
                            Ok(mesh) => {
                                let gpu_mesh = GpuMesh::from_mesh::<Vertex>(&mesh, gpu.device());
                                resources.insert(
                                    resource_id.to_string(),
                                    Resource::Mesh(MeshResource {
                                        mesh: Arc::new(gpu_mesh),
                                        transform: descriptor.transform(),
                                    }),
                                );
                            }
                            Err(error) => {
                                log::error!("Could not parse OJB source: {}", error);
                            }
                        }
                    }
                }
            }
        }

        let mut l_system_managers = HashMap::new();
        for (name, mut system) in l_systems.drain() {
            if !scene_descriptor.l_systems().contains_key(&name) {
                log::error!("System has no descriptor: {}", name);
                continue
            }
            let l_system_descriptor = scene_descriptor
                .l_systems()
                .get(&name)
                .unwrap();
            let mut instances = HashMap::new();
            for (instance_name, instance) in system.drain() {
                if !l_system_descriptor.instances().contains_key(&instance_name) {
                    log::error!("Instance has no descriptor: {}", instance_name);
                    continue
                }
                let instance_descriptor = l_system_descriptor
                    .instances()
                    .get(&instance_name)
                    .unwrap();
                let mut primitives = HashMap::new();
                for (primitive_id, primitive_descriptor) in l_system_descriptor.primitives().iter() {
                    if let Some(primitive) = resources.get(primitive_id) {
                        match primitive {
                            Resource::Mesh(mesh_primitive) => {
                                primitives.insert(
                                    primitive_id.clone(),
                                    LSystemPrimitive::new(
                                        mesh_primitive.mesh.aabb(),
                                        primitive_descriptor.transform(),
                                        primitive_descriptor.material(),
                                    )
                                );
                            }
                        }
                    }
                }
                instances.insert(
                    instance_name.to_string(),
                    LSystemManager::new(
                        instance,
                        l_system_descriptor.transform(),
                        Some(MaterialState::from(instance_descriptor)),
                        primitives,
                        instance_descriptor.tropism(),
                        gpu,
                    ),
                );
            }
            l_system_managers.insert(name, instances);
        }

        let mut objects = HashMap::new();
        for (object_id, descriptor) in scene_descriptor.scene().objects() {
            match descriptor {
                SceneObjectDescriptor::LSystem(d) => {
                    let iteration = if let Some(iteration) = d.iteration() {
                        *iteration
                    } else {
                        if !scene_descriptor.l_systems().contains_key(d.system()) {
                            log::error!("Object references unknown LSystem: {}", d.system());
                            continue
                        } else if !scene_descriptor.l_systems().get(d.system()).unwrap().instances().contains_key(d.instance()) {
                            log::error!("Object references unknown instance: {}", d.instance());
                            continue
                        }
                        scene_descriptor
                            .l_systems()
                            .get(d.system())
                            .unwrap()
                            .instances()
                            .get(d.instance())
                            .unwrap()
                            .iterations()
                    };
                    if !l_system_managers.contains_key(d.system()) {
                        log::error!("Object references unknown LSystem: {}", d.system());
                        continue
                    } else if !l_system_managers.get(d.system()).unwrap().contains_key(d.instance()) {
                        log::error!("Object references unknown instance: {}", d.instance());
                        continue
                    }
                    l_system_managers
                        .get_mut(d.system())
                        .unwrap()
                        .get_mut(d.instance())
                        .unwrap()
                        .maybe_increase_max_iteration(iteration);
                    objects.insert(
                        object_id.to_string(),
                        SceneObject {
                            transform_buffer: Buffer::new_single_element(
                                "transform buffer",
                                ModelTransform::new(d.transform().as_mat4()),
                                BufferUsages::UNIFORM,
                                gpu,
                            ),
                            primitive: Primitive::LSystem(LSystemObject {
                                system: d.system().to_string(),
                                instance: d.instance().to_string(),
                                target_iteration: iteration,
                                active_iteration: None,
                                render_objects: HashMap::new(),
                            }),
                        },
                    );
                }
                SceneObjectDescriptor::Obj(d) => {
                    if !resources.contains_key(d.obj()) {
                        log::error!("Object references unknown mesh: {}", d.obj());
                        continue
                    }
                    let mesh = resources.get(d.obj()).unwrap();
                    objects.insert(
                        object_id.to_string(),
                        SceneObject {
                            transform_buffer: Buffer::new_single_element(
                                "transform buffer",
                                ModelTransform::new(d.transform().as_mat4()),
                                BufferUsages::UNIFORM,
                                gpu,
                            ),
                            primitive: Primitive::Mesh(SceneMesh {
                                mesh: mesh.mesh().clone(),
                                instance_buffer: Buffer::new_single_element(
                                    "instance buffer",
                                    Instance::new(mesh.transform().as_mat4(), d.material()),
                                    BufferUsages::STORAGE,
                                    gpu,
                                ),
                                render_objects: None,
                            }),
                        },
                    );
                }
            }
        }

        Self {
            background_color: scene_descriptor.scene().camera().background_color(),
            camera,
            aspect_ratio: view_aspect_ratio,
            ambient_light,
            light_sources,
            light_sources_bind_group: None,
            objects,
            cylinder_mesh: l_system_cylinder_mesh,
            resources,
            l_systems: l_system_managers,
        }
    }

    pub fn get_active_render_objects(&self) -> Vec<&Vec<RenderObject>> {
        self.objects
            .iter()
            .filter_map(|(_, o)| match &o.primitive {
                Primitive::LSystem(l_system) => {
                    if !l_system.render_objects.is_empty() {
                        l_system
                            .render_objects
                            .get(&l_system.active_iteration.unwrap())
                    } else {
                        None
                    }
                }
                Primitive::Mesh(mesh) => mesh.render_objects.as_ref(),
            })
            .collect()
    }

    pub fn get_light_sources_bind_group(&self) -> &LightSourcesBindGroup {
        self.light_sources_bind_group
            .as_ref()
            .expect("Light sources bind group not initialized")
    }

    pub fn prepare_render(
        &mut self,
        render_object_creator: &RenderObjectBuilder,
        light_sources_bind_group_creator: &LightSourcesBindGroupBuilder,
    ) {
        if self.light_sources_bind_group.is_none() {
            self.light_sources_bind_group =
                Some(light_sources_bind_group_creator.build(self.ambient_light.light(), self.lights().as_slice()));
        }

        for (_, o) in self.objects.iter_mut() {
            match &mut o.primitive {
                Primitive::LSystem(l_system) => {
                    if !l_system
                        .render_objects
                        .contains_key(&l_system.target_iteration)
                    {
                        let iteration = self
                            .l_systems
                            .get(&l_system.system)
                            .unwrap_or_else(|| panic!("Unknown system: {}", l_system.system))
                            .get(&l_system.instance)
                            .unwrap_or_else(|| panic!("Unkown instance: {}", l_system.instance))
                            .try_get_iteration(l_system.target_iteration);
                        let insert = if let Some(active_iteration) = l_system.active_iteration {
                            active_iteration != iteration.0
                        } else {
                            true
                        };
                        if insert {
                            let cylinder_render_object = render_object_creator
                                .build(
                                    &self.cylinder_mesh,
                                    &o.transform_buffer,
                                    iteration.1.cylinder_instances_buffer(),
                                );

                            let mut render_objects = vec![cylinder_render_object];
                            for (primitive_id, primitive_instances) in
                                iteration.1.primitive_instances().iter()
                            {
                                if let Some(resource) = self.resources.get_mut(primitive_id) {
                                    for (iteration, instance_buffer) in primitive_instances.iter() {
                                        render_objects.push(
                                            render_object_creator.build(
                                                resource.get_or_create_mesh(iteration),
                                                &o.transform_buffer,
                                                instance_buffer,
                                            ),
                                        );
                                    }
                                }
                            }

                            l_system.render_objects.insert(iteration.0, render_objects);
                            l_system.active_iteration = Some(iteration.0)
                        }
                    }
                }
                Primitive::Mesh(mesh) => {
                    if mesh.render_objects.is_none() {
                        mesh.render_objects = Some(vec![render_object_creator
                            .build(
                                &mesh.mesh,
                                &o.transform_buffer,
                                &mesh.instance_buffer,
                            )]);
                    }
                }
            };
        }
    }

    pub fn set_target_iteration(&mut self, object_name: &str, target_iteration: u32) {
        if let Some(object) = self.objects.get_mut(object_name) {
            match &mut object.primitive {
                Primitive::LSystem(l_system) => {
                    l_system.target_iteration = target_iteration;
                    if l_system.render_objects.contains_key(&target_iteration) {
                        l_system.active_iteration = Some(target_iteration);
                    }
                }
                _ => log::warn!("Can not set target iteration on non L-System object"),
            }
        } else {
            log::warn!("Unknown object: {}", object_name);
        }
    }

    pub fn camera(&self) -> OrbitCamera {
        self.camera
    }
    pub fn lights(&self) -> &Vec<LightSource> {
        &self.light_sources
    }
    pub fn background_color(&self) -> Vec3 {
        self.background_color
    }
    pub fn set_background_color(&mut self, background_color: Vec3) {
        self.background_color = background_color;
    }
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    pub fn ambient_light(&self) -> LightSource {
        self.ambient_light
    }
}

impl Update for LSystemScene {
    fn update(&mut self, input: &Input) {
        self.camera.update(input);
        for (_, system) in self.l_systems.iter_mut() {
            for (_, instance) in system.iter_mut() {
                instance.update(input);
            }
        }
    }
}

impl OnResize for LSystemScene {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
        self.camera.on_resize(width, height);
    }
}

impl Drop for LSystemScene {
    fn drop(&mut self) {
        for (_, o) in self.objects.iter_mut() {
            o.transform_buffer.buffer().destroy();
        }
    }
}
