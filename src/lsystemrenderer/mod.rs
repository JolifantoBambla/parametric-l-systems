use std::sync::Arc;
use wgpu::{Backends, BufferUsages, DownlevelCapabilities, DownlevelFlags, Limits, ShaderModel, SurfaceConfiguration, TextureView};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use crate::framework::app::GpuApp;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::{ContextDescriptor, Gpu, SurfaceContext};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::Vertex;
use crate::framework::renderer::drawable::GpuMesh;
use crate::framework::scene::Update;
use crate::framework::util::window::Resize;
use crate::lsystemrenderer::camera::OrbitCamera;

pub mod camera;

pub struct App {
    ctx: Arc<Gpu>,
    camera: OrbitCamera,
    camera_uniforms: Buffer<camera::Uniforms>,
    cylinder_mesh: GpuMesh,
}

impl App {
    pub fn new(ctx: &Arc<Gpu>, surface_configuration: &SurfaceConfiguration) -> Self {
        let camera = OrbitCamera::new(
            Projection::new_perspective(
                f32::to_radians(45.),
                surface_configuration.width as f32 / surface_configuration.height as f32,
                0.0001,
                1000.0
            ),
            CameraView::default(),
            5.0
        );
        let cylinder_mesh = GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(false),
            ctx.device()
        );
        let camera_uniforms = Buffer::new_single_element(
            "camera uniforms",
            camera.as_uniforms(),
            BufferUsages::UNIFORM,
            ctx,
        );
        Self {
            ctx: ctx.clone(),
            camera,
            camera_uniforms,
            cylinder_mesh
        }
    }
}

impl GpuApp<()> for App {
    fn init(&mut self, event_loop: &EventLoop<()>, context: &SurfaceContext) {}

    fn on_user_event(&mut self, _event: &()) {}

    fn on_window_event(&mut self, _event: &WindowEvent) {}

    fn render(&mut self, view: &TextureView, input: &Input) {
        log::info!("render called - frame {}", input.frame().number());

        self.camera_uniforms.write_buffer(&vec![self.camera.as_uniforms()]);
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
