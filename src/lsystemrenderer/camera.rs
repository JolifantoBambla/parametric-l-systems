use glam::Mat4;
use crate::framework::camera;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::input::Input;
use crate::framework::scene::Update;
use crate::framework::util::window::Resize;

pub struct Camera {
    projection: Projection,
    transform: CameraView,
    speed: f32,
}

impl Camera {
    pub fn new(projection: Projection, transform: CameraView, speed: f32) -> Self {
        Self { projection, transform, speed }
    }
}

impl camera::Camera for Camera {
    fn view(&self) -> Mat4 {
        self.transform.view()
    }
    fn projection(&self) -> Mat4 {
        self.projection.projection()
    }
}

impl Resize for Camera {
    fn resize(&mut self, width: u32, height: u32) {
        self.projection.resize(width, height)
    }
}

impl Update for Camera {
    fn update(&mut self, input: &Input) {

        //todo!()
    }
}
