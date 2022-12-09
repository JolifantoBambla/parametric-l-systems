use std::sync::Arc;
use glam::Vec3;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::Gpu;
use crate::framework::scene::light::PointLight;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::turtle::turtle::LSystemManager;

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
        let model = LSystemManager::new(l_system, &instances_bind_group_layout, 1, gpu);
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
}
