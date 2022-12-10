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
use glam::Vec3;
use std::sync::Arc;

pub struct LSystemScene {
    camera: OrbitCamera,
    light_sources: Vec<LightSource>,
    model: LSystemManager,
    light_sources_bind_group: Option<LightSourcesBindGroup>,
}

impl LSystemScene {
    pub fn new(
        l_system: LSystem,
        light_sources: Vec<LightSource>,
        view_aspect_ratio: f32,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let camera = OrbitCamera::new(
            Projection::new_perspective(f32::to_radians(45.), view_aspect_ratio, 0.0001, 1000.0),
            CameraView::new(Vec3::new(0., 0., -10.), Vec3::ZERO, Vec3::Y),
            5.0,
        );
        let model = LSystemManager::new(l_system, gpu);
        Self {
            camera,
            model,
            light_sources,
            light_sources_bind_group: None,
        }
    }
    pub fn camera(&self) -> OrbitCamera {
        self.camera
    }
    pub fn lights(&self) -> &Vec<LightSource> {
        &self.light_sources
    }
    pub fn model(&self) -> &LSystemManager {
        &self.model
    }
    pub fn get_active_render_objects(&self) -> &Vec<RenderObject> {
        self.model.get_render_objects()
    }

    pub fn get_light_sources_bind_group(&self) -> &LightSourcesBindGroup {
        self.light_sources_bind_group.as_ref()
            .expect("Light sources bind group not initialized")
    }

    pub fn prepare_render(&mut self, render_object_creator: &RenderObjectCreator, light_sources_bind_group_creator: &LightSourcesBindGroupCreator) {
        self.model.prepare_render(render_object_creator);
        if self.light_sources_bind_group.is_none() {
            self.light_sources_bind_group = Some(light_sources_bind_group_creator.create(self.lights()));
        }
    }

    pub fn set_target_iteration(&mut self, target_iteration: u32) {
        self.model.set_target_iteration(target_iteration);
    }
}

impl Update for LSystemScene {
    fn update(&mut self, input: &Input) {
        self.camera.update(input);
        self.model.update(input);
    }
}

impl OnResize for LSystemScene {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.camera.on_resize(width, height);
    }
}
