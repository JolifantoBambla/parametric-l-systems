use std::collections::HashMap;
use glam::{Quat, Vec3};
use std::sync::Arc;
use wasm_bindgen::JsValue;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::input::Input;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::GpuMesh;
use crate::framework::scene::light::LightSource;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::renderer::{LightSourcesBindGroup, LightSourcesBindGroupCreator, RenderObject, RenderObjectCreator};
use crate::lsystemrenderer::turtle::turtle::LSystemManager;
use crate::framework::scene::transform::Transform;
use crate::lsystemrenderer::scene_descriptor::{LSystemSceneDescriptor, SceneObjectDescriptor};

struct Mesh {
    mesh: GpuMesh,
    transform: Transform,
}

struct LSystemObject {
    system: String,
    instance: String,
    iteration: usize,
}

enum Primitive {
    LSystem(LSystemObject)
}

struct SceneObject {
    transform: Transform,
    primitive: Primitive,
}

impl Update for SceneObject {
    fn update(&mut self, input: &Input) {
        match &mut self.primitive {
            Primitive::LSystem(l_system) => l_system.update(input)
        };
    }
}

pub struct LSystemScene {
    background_color: Vec3,
    camera: OrbitCamera,
    ambient_light: LightSource,
    light_sources: Vec<LightSource>,
    light_sources_bind_group: Option<LightSourcesBindGroup>,
    objects: Vec<SceneObject>,
    cylinder_mesh: Arc<Mesh>,
    meshes: HashMap<String, Arc<Mesh>>,
    l_systems: HashMap<String, HashMap<String, LSystemManager>>,
}

impl LSystemScene {
    pub fn new(
        mut l_systems: HashMap<String, HashMap<String, LSystem>>,
        scene_descriptor: LSystemSceneDescriptor,
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
            light_sources.push(LightSource::new_directional(descriptor.direction(), descriptor.color(), 1.0));
        }
        for descriptor in scene_descriptor.scene().lights().point_lights() {
            light_sources.push(LightSource::new_point(descriptor.position(), descriptor.color(), 1.0));
        }

        let l_system_cylinder_mesh = Arc::new(
            Mesh {
                mesh: GpuMesh::from_mesh::<Vertex>(
                    &Mesh::new_default_cylinder(true),
                    gpu.device(),
                ),
                transform: Transform::from_rotation(Quat::from_rotation_x(f32::to_radians(-90.)))
            },
        );
        // todo: parse obj resources into meshes
        let meshes = HashMap::new();

        let mut l_system_managers = HashMap::new();
        for (name, mut system) in l_systems.drain() {
            let mut instances = HashMap::new();
            for (instance_name, instance) in system.drain() {
                // todo: this also needs some data from scene_descriptor
                instances.insert(instance_name.to_string(), LSystemManager::new(
                    instance,
                    scene_descriptor.l_system_settings(),
                    gpu)
                );
            }
            l_system_managers.insert(name, instances);
        }

        let mut objects = Vec::new();
        for descriptor in scene_descriptor.scene().objects() {
            match descriptor {
                SceneObjectDescriptor::LSystem(d) => {
                    // todo: set system's target iteration to max(d.iteration(), systen.target_iteration)
                    objects.push(SceneObject {
                        transform: d.transform(),
                        primitive: Primitive::LSystem(LSystemObject{
                            system: d.system().to_string(),
                            instance: d.instance().to_string(),
                            iteration: d.iteration(),
                        })
                    });
                }
                SceneObjectDescriptor::Obj(_) => log::warn!("Obj object type not yet supported"),
            }
        }

        Self {
            background_color: scene_descriptor.scene().camera().background_color(),
            camera,
            ambient_light,
            light_sources,
            light_sources_bind_group: None,
            objects,
            meshes,
            l_systems: l_system_managers,
        }
    }


    pub fn get_active_render_objects(&self) -> Vec<&Vec<RenderObject>> {
        self.objects.iter()
            .map(|o| match &o.primitive {
                Primitive::LSystem(l_system) => {
                    l_system.get_render_objects()
                }
            })
            .collect()
    }

    pub fn get_light_sources_bind_group(&self) -> &LightSourcesBindGroup {
        self.light_sources_bind_group.as_ref()
            .expect("Light sources bind group not initialized")
    }

    pub fn prepare_render(&mut self, render_object_creator: &RenderObjectCreator, light_sources_bind_group_creator: &LightSourcesBindGroupCreator) {
        self.objects.iter_mut()
            .for_each(|o| match &mut o.primitive {
                Primitive::LSystem(l_system) => {
                    l_system.prepare_render(render_object_creator);
                }
            });
        if self.light_sources_bind_group.is_none() {
            self.light_sources_bind_group = Some(light_sources_bind_group_creator.create(self.lights()));
        }
    }

    // todo: set iteration for specific l_System
    pub fn set_target_iteration(&mut self, target_iteration: u32) {
        self.objects.iter_mut()
            .for_each(|o| match &mut o.primitive {
                Primitive::LSystem(l_system) => {
                    l_system.set_target_iteration(target_iteration);
                }
            });
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
}

impl Update for LSystemScene {
    fn update(&mut self, input: &Input) {
        self.camera.update(input);
        self.objects.iter_mut().for_each(|o| o.update(input));
    }
}

impl OnResize for LSystemScene {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.camera.on_resize(width, height);
    }
}
