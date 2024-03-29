use crate::framework::input::Input;
use wgpu::SubmissionIndex;

pub trait Update {
    fn update(&mut self, input: &Input);
}

pub trait PrepareRender {
    fn prepare_render(&mut self, input: &Input);
}

pub trait OnCommandsSubmitted {
    fn on_commands_submitted(&mut self, input: &Input, submission_index: &SubmissionIndex);
}
