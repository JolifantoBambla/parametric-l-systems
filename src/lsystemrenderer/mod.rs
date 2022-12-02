use std::sync::Arc;
use wgpu::{Backends, DownlevelCapabilities, DownlevelFlags, Limits, ShaderModel, SurfaceConfiguration, TextureView};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use crate::framework::app::GpuApp;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::{ContextDescriptor, Gpu, SurfaceContext};
use crate::framework::input::Input;
use crate::framework::scene::Update;
use crate::framework::util::window::Resize;
use crate::lsystemrenderer::camera::Camera;

pub mod camera;

pub struct App {
    ctx: Arc<Gpu>,
    camera: Camera,
}

impl App {
    pub fn new(ctx: &Arc<Gpu>, surface_configuration: &SurfaceConfiguration) -> Self {
        let camera = Camera::new(
            Projection::new_perspective(
                f32::to_radians(45.),
                surface_configuration.width as f32 / surface_configuration.height as f32,
                0.0001,
                1000.0
            ),
            CameraView::default(),
            5.0
        );
        Self { ctx: ctx.clone(), camera }
    }
}

impl GpuApp<()> for App {
    fn init(&mut self, event_loop: &EventLoop<()>, context: &SurfaceContext) {}

    fn on_user_event(&mut self, _event: &()) {}

    fn on_window_event(&mut self, _event: &WindowEvent) {}

    fn render(&mut self, view: &TextureView, input: &Input) {
        log::info!("render called - frame {}", input.frame().number());
    }

    fn get_context_descriptor() -> ContextDescriptor<'static> {
        // todo: check if webgl needs other defaults
        ContextDescriptor {
            required_limits: Limits::downlevel_webgl2_defaults(),
            required_downlevel_capabilities: DownlevelCapabilities {
                flags: DownlevelFlags::empty(),
                shader_model: ShaderModel::Sm5,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl Update for App {
    fn update(&mut self, input: &Input) {
        self.camera.update(input);
    }
}

impl Resize for App {
    fn resize(&mut self, width: u32, height: u32) {
        self.camera.resize(width, height);
    }
}
