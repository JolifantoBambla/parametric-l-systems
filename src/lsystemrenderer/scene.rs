use std::collections::HashMap;
use glam::Vec3;
use std::sync::Arc;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::input::Input;
use crate::framework::scene::light::LightSource;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::renderer::{LightSourcesBindGroup, LightSourcesBindGroupCreator, RenderObject, RenderObjectCreator};
use crate::lsystemrenderer::turtle::turtle::LSystemManager;
use crate::framework::scene::transform::Transform;
use crate::SceneDescriptor;

enum Primitive {
    LSystem(LSystemManager)
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
    camera: OrbitCamera,
    light_sources: Vec<LightSource>,
    light_sources_bind_group: Option<LightSourcesBindGroup>,
    objects: Vec<SceneObject>,

    l_systems: HashMap<String, HashMap<String, LSystemManager>>,
}

impl LSystemScene {
    pub fn new(
        l_system: LSystem,
        //l_systems: HashMap<String, HashMap<String, LSystem>>,
        scene_descriptor: SceneDescriptor,
        view_aspect_ratio: f32,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let camera = OrbitCamera::new(
            Projection::new_perspective(f32::to_radians(45.), view_aspect_ratio, 0.0001, 1000.0),
            CameraView::new(Vec3::new(0., 0., -10.), Vec3::ZERO, Vec3::Y),
            5.0,
        );


        /*
        let mut l_systems = HashMap::new();
        for (name, system) in l_systems.iter() {
            let mut instances = HashMap::new();
            for (instance_name, instance) in instances.iter() {
                instances.insert(instance_name, instance)
            }
            l_systems.insert(name, instances);
        }

         */

        let scene_object = SceneObject {
            transform: Transform::default(),
            primitive: Primitive::LSystem(LSystemManager::new(l_system, scene_descriptor.l_system_settings(), gpu))
        };

        Self {
            camera,
            light_sources: scene_descriptor.light_sources().clone(),
            light_sources_bind_group: None,
            objects: vec![scene_object],
            l_systems: HashMap::new(),
        }
    }
    pub fn camera(&self) -> OrbitCamera {
        self.camera
    }
    pub fn lights(&self) -> &Vec<LightSource> {
        &self.light_sources
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
