use crate::framework::input::event_buffer::EventBuffer;
use crate::framework::input::frame::Frame;
use crate::framework::input::time::Time;

pub mod event_buffer;
pub mod frame;
pub mod keyboard;
pub mod time;

#[derive(Copy, Clone, Debug, Default)]
pub struct Input {
    time: Time,
    frame: Frame,
}

impl Input {
    pub fn next(&self, event_buffer: EventBuffer) -> Self {
        Self {
            time: self.time.next(),
            frame: self.frame.next(),
        }
    }
    pub fn time(&self) -> Time {
        self.time
    }
    pub fn frame(&self) -> Frame {
        self.frame
    }
}
