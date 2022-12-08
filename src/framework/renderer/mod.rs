use crate::framework::camera::Camera;
use crate::framework::input::Input;
use crate::framework::scene::Scene;
use wgpu::{CommandEncoder, TextureView};

pub mod drawable;

// todo: refactor / remove
pub mod trivial_renderer;

pub trait Renderer {
    fn render(
        &self,
        render_target: &TextureView,
        camera: &dyn Camera,
        input: &Input,
        scene: &Scene,
        command_encoder: &mut CommandEncoder,
    );
}
