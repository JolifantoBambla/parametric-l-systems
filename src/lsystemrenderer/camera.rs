use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::input::mouse::MouseEvent;
use crate::framework::input::{Event, Input};
use crate::framework::scene::camera::{Camera, CameraView, Projection};
use crate::framework::scene::transform::util::Orbit;
use crate::framework::scene::transform::{Transform, Transformable};
use glam::{Mat4, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct OrbitCamera {
    camera: Camera,
    speed: f32,
}

impl OrbitCamera {
    pub fn new(projection: Projection, transform: CameraView, speed: f32) -> Self {
        Self {
            camera: Camera::new(transform, projection),
            speed,
        }
    }
    pub fn view(&self) -> Mat4 {
        self.camera.view_mat()
    }
    pub fn projection(&self) -> Mat4 {
        self.camera.projection_mat()
    }
}

impl Transformable for OrbitCamera {
    fn transform(&self) -> &Transform {
        self.camera.transform()
    }

    fn transform_mut(&mut self) -> &mut Transform {
        self.camera.transform_mut()
    }
}

impl Orbit for OrbitCamera {
    fn target(&self) -> Vec3 {
        self.camera.view().center_of_projection()
    }

    fn set_target(&mut self, target: Vec3) {
        self.camera.view().set_center_of_projection(target);
    }
}

impl OnResize for OrbitCamera {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.camera.projection().on_resize(width, height)
    }
}

impl Update for OrbitCamera {
    fn update(&mut self, input: &Input) {
        for e in input.events() {
            match e {
                Event::Mouse(m) => match m {
                    MouseEvent::Move(m) => {
                        if m.state().left_button_pressed() {
                            self.orbit(m.delta(), false);
                        } else if m.state().right_button_pressed() {
                            let translation = m.delta() * self.speed * 20.;
                            self.camera.move_right(translation.x);
                            self.camera.move_down(translation.y);
                        }
                    }
                    MouseEvent::Scroll(s) => {
                        self.camera
                            .zoom_in(s.delta().abs().min(1.) * s.delta().signum());
                    }
                    _ => {}
                },
            }
        }
    }
}
