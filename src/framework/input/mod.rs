use crate::framework::event::window::{OnResize, OnWindowEvent};
use crate::framework::input::frame::Frame;
use crate::framework::input::mouse::{Mouse, MouseEvent};
use crate::framework::input::time::Time;
use winit::event::WindowEvent;

pub mod frame;
pub mod keyboard;
pub mod mouse;
pub mod time;

#[derive(Clone, Debug)]
pub enum Event {
    Mouse(MouseEvent),
}

#[derive(Clone, Debug)]
pub struct Input {
    time: Time,
    frame: Frame,
    mouse: Mouse,
    events: Vec<Event>,
}

impl Input {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            time: Time::default(),
            frame: Frame::default(),
            mouse: Mouse::new(width, height),
            events: Vec::new(),
        }
    }

    pub fn prepare_next(&mut self) -> Self {
        let last = self.clone();

        self.time = self.time.next();
        self.frame = self.frame.next();
        self.mouse = self.mouse.next();
        self.events = Vec::new();

        last
    }
    pub fn time(&self) -> Time {
        self.time
    }
    pub fn frame(&self) -> Frame {
        self.frame
    }
    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
}

impl OnResize for Input {
    fn on_resize(&mut self, width: u32, height: u32) {
        self.mouse.on_resize(width, height);
    }
}

impl OnWindowEvent for Input {
    fn on_window_event(&mut self, event: &WindowEvent) {
        if let Some(e) = self.mouse.handle_event(event) {
            self.events.push(Event::Mouse(e));
        }
    }
}
