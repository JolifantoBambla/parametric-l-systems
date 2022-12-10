use std::sync::Arc;
use glam::Vec3;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::input::Input;
use crate::framework::scene::light::PointLight;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::renderer::{RenderObjectCreator, RenderObject};
use crate::lsystemrenderer::turtle::turtle::{LSystemManager, LSystemModel};

pub struct LSystemScene {
    camera: OrbitCamera,
    lights: Vec<PointLight>,
    model: LSystemManager,
}

impl LSystemScene {
    pub fn new(l_system: LSystem, lights: Vec<PointLight>, view_aspect_ratio: f32, gpu: &Arc<Gpu>) -> Self {
        let camera = OrbitCamera::new(
            Projection::new_perspective(
                f32::to_radians(45.),
                view_aspect_ratio,
                0.0001,
                1000.0,
            ),
            CameraView::new(Vec3::new(0., 0., -10.), Vec3::ZERO, Vec3::Y),
            5.0,
        );
        let model = LSystemManager::new(l_system, gpu);
        Self {
            camera,
            model,
            lights,
        }
    }
    pub fn camera(&self) -> OrbitCamera {
        self.camera
    }
    pub fn lights(&self) -> &Vec<PointLight> {
        &self.lights
    }
    pub fn model(&self) -> &LSystemManager {
        &self.model
    }
    pub fn get_active_render_objects(&self) -> &Vec<RenderObject> {
        self.model.get_render_objects()
    }

    pub fn prepare_render(&mut self, render_object_creator: &RenderObjectCreator) {
        self.model.prepare_render(render_object_creator);
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
