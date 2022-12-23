use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::GpuMesh;
use crate::framework::scene::light::LightSource;
use crate::framework::scene::transform::Transform;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::renderer::{
    LightSourcesBindGroup, LightSourcesBindGroupCreator, RenderObject, RenderObjectCreator,
};
use crate::lsystemrenderer::scene_descriptor::{LSystemSceneDescriptor, SceneObjectDescriptor};
use crate::lsystemrenderer::turtle::turtle::{Instance, LSystemManager, MaterialState};
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::BufferUsages;

struct SceneMesh {
    mesh: Arc<GpuMesh>,
    transform: Transform,
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
    transform_buffer: Buffer<Mat4>,
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
    meshes: HashMap<String, Arc<SceneMesh>>,
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
            5.0,
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
                1.0,
            ));
        }
        for descriptor in scene_descriptor.scene().lights().point_lights() {
            light_sources.push(LightSource::new_point(
                descriptor.position(),
                descriptor.color(),
                1.0,
            ));
        }

        let l_system_cylinder_mesh = Arc::new(GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(true),
            gpu.device(),
        ));
        // todo: parse obj resources into meshes
        let meshes = HashMap::new();

        let mut l_system_managers = HashMap::new();
        for (name, mut system) in l_systems.drain() {
            let system_descriptor = scene_descriptor
                .systems()
                .get(&name)
                .unwrap_or_else(|| panic!("System has no descriptor: {}", name));
            let mut instances = HashMap::new();
            for (instance_name, instance) in system.drain() {
                let instance_descriptor = system_descriptor
                    .instances()
                    .get(&instance_name)
                    .unwrap_or_else(|| panic!("Instance has no descriptor: {}", instance_name));
                instances.insert(
                    instance_name.to_string(),
                    LSystemManager::new(
                        instance,
                        system_descriptor.transform(),
                        Some(MaterialState::from(instance_descriptor)),
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
                        scene_descriptor
                            .systems()
                            .get(d.system())
                            .unwrap_or_else(|| {
                                panic!("Object references unknown LSystem: {}", d.system())
                            })
                            .instances()
                            .get(d.instance())
                            .unwrap_or_else(|| {
                                panic!("Object references unknown instance: {}", d.instance())
                            })
                            .iterations()
                    };
                    l_system_managers
                        .get_mut(d.system())
                        .unwrap_or_else(|| {
                            panic!("Object references unknown LSystem: {}", d.system())
                        })
                        .get_mut(d.instance())
                        .unwrap_or_else(|| {
                            panic!("Object references unknown instance: {}", d.instance())
                        })
                        .maybe_increase_max_iteration(iteration);
                    objects.insert(
                        object_id.to_string(),
                        SceneObject {
                            transform_buffer: Buffer::new_single_element(
                                "transform buffer",
                                d.transform().as_mat4(),
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
                SceneObjectDescriptor::Obj(_) => log::warn!("Obj object type not yet supported"),
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
            meshes,
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
                Primitive::Mesh(mesh) => {
                    mesh.render_objects.as_ref()
                }
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
        render_object_creator: &RenderObjectCreator,
        light_sources_bind_group_creator: &LightSourcesBindGroupCreator,
    ) {
        if self.light_sources_bind_group.is_none() {
            self.light_sources_bind_group =
                Some(light_sources_bind_group_creator.create(self.lights().as_slice()));
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
                                .create_render_object(
                                    &self.cylinder_mesh,
                                    &o.transform_buffer,
                                    iteration.1.cylinder_instances_buffer(),
                                );
                            // todo: add other meshes that are used by the iteration
                            l_system
                                .render_objects
                                .insert(iteration.0, vec![cylinder_render_object]);
                            l_system.active_iteration = Some(iteration.0)
                        }
                    }
                }
                Primitive::Mesh(mesh) => {
                    if mesh.render_objects.is_none() {
                        mesh.render_objects = Some(vec![render_object_creator.create_render_object(
                            &mesh.mesh, &o.transform_buffer, &mesh.instance_buffer
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
