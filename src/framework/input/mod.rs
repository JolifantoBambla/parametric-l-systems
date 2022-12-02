use winit::event::WindowEvent;
use winit::window::CursorIcon::Default;
use crate::framework::input::frame::Frame;
use crate::framework::input::mouse::Mouse;
use crate::framework::input::time::Time;
use crate::framework::util::window::Resize;

pub mod frame;
pub mod keyboard;
pub mod mouse;
pub mod time;

#[derive(Clone, Debug)]
pub struct Input {
    time: Time,
    frame: Frame,
    mouse: Mouse,
}

impl Input {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            time: Time::default(),
            frame: Frame::default(),
            mouse: Mouse::new(width, height),
        }
    }

    pub fn prepare_next(&mut self) -> Self {
        let last = self.clone();

        self.time = self.time.next();
        self.frame = self.frame.next();
        self.mouse = self.mouse.next();

        last
    }
    pub fn time(&self) -> Time {
        self.time
    }
    pub fn frame(&self) -> Frame {
        self.frame
    }
    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.mouse.handle_event(event);
    }
}

impl Resize for Input {
    fn resize(&mut self, width: u32, height: u32) {
        self.mouse.resize(width, height);
    }
}
