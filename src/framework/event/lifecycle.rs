use wgpu::SubmissionIndex;
use crate::framework::input::Input;

pub trait OnUpdate {
    fn on_update(&mut self, input: &Input);
}

pub trait OnCommandsSubmitted {
    fn on_commands_submitted(&mut self, input: &Input, submission_index: &SubmissionIndex);
}
