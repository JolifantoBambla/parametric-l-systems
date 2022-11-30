use std::collections::VecDeque;
use winit::event::WindowEvent;

#[derive(Copy, Clone, Debug, Default)]
pub struct EventBuffer {}

impl EventBuffer {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        // todo: handle inputs
    }

    pub fn drain(&mut self) -> Self {
        let snap_shot = self.clone();
        // todo: empty buffer
        snap_shot
    }
}