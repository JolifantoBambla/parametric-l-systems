use crate::framework::camera::{Camera, CameraView, Projection};
use crate::framework::event::lifecycle::Update;
use crate::framework::event::window::OnResize;
use crate::framework::input::mouse::MouseEvent;
use crate::framework::input::{Event, Input};
use glam::Mat4;

#[derive(Copy, Clone, Debug)]
pub struct OrbitCamera {
    projection: Projection,
    transform: CameraView,
    speed: f32,
}

impl OrbitCamera {
    pub fn new(projection: Projection, transform: CameraView, speed: f32) -> Self {
        Self {
            projection,
            transform,
            speed,
        }
    }
}

impl Camera for OrbitCamera {
    fn view(&self) -> Mat4 {
        self.transform.view()
    }
    fn projection(&self) -> Mat4 {
        self.projection.projection()
    }
}

impl OnResize for OrbitCamera {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.projection.on_resize(width, height)
    }
}

impl Update for OrbitCamera {
    fn update(&mut self, input: &Input) {
        for e in input.events() {
            match e {
                Event::Mouse(m) => match m {
                    MouseEvent::Move(m) => {
                        if m.state().left_button_pressed() {
                            self.transform.orbit(m.delta(), false);
                        } else if m.state().right_button_pressed() {
                            let translation = m.delta() * self.speed * 20.;
                            self.transform.move_right(translation.x);
                            self.transform.move_down(translation.y);
                        }
                    }
                    MouseEvent::Scroll(s) => {
                        self.transform
                            .zoom_in(s.delta().abs().min(1.) * s.delta().signum());
                    }
                    _ => {}
                },
            }
        }
    }
}
