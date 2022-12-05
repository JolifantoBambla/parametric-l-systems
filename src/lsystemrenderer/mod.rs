use std::borrow::Cow;
use std::mem;
use std::sync::Arc;
use glam::{Mat4, Vec3};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
use wgpu::{Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Color, ColorTargetState, CommandEncoder, CommandEncoderDescriptor, CompareFunction, DepthStencilState, DownlevelCapabilities, DownlevelFlags, Extent3d, FragmentState, Label, Limits, LoadOp, Operations, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModel, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, VertexState};
use wgpu::Face::Back;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;
use crate::framework::app::GpuApp;
use crate::framework::camera::{CameraView, Projection};
use crate::framework::context::{ContextDescriptor, Gpu, SurfaceContext};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::mesh::mesh::Mesh;
use crate::framework::mesh::vertex::{Vertex, VertexType};
use crate::framework::renderer::drawable::{Draw, GpuMesh};
use crate::framework::scene::Update;
use crate::framework::util::window::Resize;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::camera::{OrbitCamera, Uniforms};
use crate::lsystemrenderer::event::{UiEvent, register_ui_event_handler, LSystemEvent};

pub mod camera;
pub mod event;

pub struct App {
    ctx: Arc<Gpu>,
    camera: OrbitCamera,

    l_system: LSystem,

    cylinder_mesh: GpuMesh,

    // render stuff
    camera_uniforms: Buffer<camera::Uniforms>,
    depth_view: TextureView,
    render_pipeline: RenderPipeline,
    bind_group: BindGroup,
}

impl App {
    pub fn new(ctx: &Arc<Gpu>, surface_configuration: &SurfaceConfiguration, l_system: LSystem) -> Self {
        let width = surface_configuration.width;
        let height = surface_configuration.height;

        let camera = OrbitCamera::new(
            Projection::new_perspective(
                f32::to_radians(45.),
                width as f32 / height as f32,
                0.0001,
                1000.0
            ),
            CameraView::new(
                Vec3::new(0., 0., -10.),
                Vec3::ZERO,
                Vec3::Y,
            ),
            5.0
        );

        let cylinder_mesh = GpuMesh::from_mesh::<Vertex>(
            &Mesh::new_default_cylinder(false),
            ctx.device()
        );

        let camera_uniforms = Buffer::new_single_element(
            "camera uniforms",
            camera.as_uniforms(),
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            ctx,
        );
        let instances = vec![Mat4::IDENTITY, Mat4::from_translation(Vec3::X)];
        let instances_buffer = Buffer::from_data(
            "instances uniform",
            &instances,
            BufferUsages::STORAGE | BufferUsages::COPY_DST,
            ctx
        );

        let depth_format = TextureFormat::Depth32Float;
        let depth_texture = ctx.device().create_texture(&TextureDescriptor {
            label: Label::from("depth texture"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT,
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let shader_module = ctx.device().create_shader_module(ShaderModuleDescriptor {
                label: Label::from("shader module"),
                source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let camera_uniforms_bind_group_layout =
            ctx.device()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(
                                    mem::size_of::<Uniforms>() as _,
                                ),
                            },
                            count: None
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(
                                    mem::size_of::<Mat4>() as _,
                                ),
                            },
                            count: None
                        }
                    ],
                });
        let vertex_buffer_layouts = vec![Vertex::buffer_layout()];
        let pipeline_layout = ctx.device().create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Label::from("render pipeline layout"),
            bind_group_layouts: &[&camera_uniforms_bind_group_layout],
            push_constant_ranges: &[]
        });
        let render_pipeline = ctx.device().create_render_pipeline(&RenderPipelineDescriptor {
            label: Label::from("render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vertex_main",
                buffers: vertex_buffer_layouts.as_slice(),
            },
            primitive: PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Back),
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default()
            }),
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fragment_main",
                targets: &[Some(surface_configuration.format.into())],
            }),
            multiview: None
        });

        let bind_group = ctx.device().create_bind_group(&BindGroupDescriptor {
            label: Label::from("render pipeline bind group"),
            layout: &camera_uniforms_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_uniforms.buffer().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: instances_buffer.buffer().as_entire_binding(),
                }
            ]
        });

        Self {
            ctx: ctx.clone(),
            l_system,
            camera,
            cylinder_mesh,
            camera_uniforms,
            depth_view,
            render_pipeline,
            bind_group,
        }
    }

    fn render_inner(&self, view: &TextureView, command_encoder: &mut CommandEncoder) {
        self.camera_uniforms.write_buffer(&vec![self.camera.as_uniforms()]);

        let color_attachment = RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0}),
                store: true,
            }
        };
        let depth_attachment = RenderPassDepthStencilAttachment {
            view: &self.depth_view,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: false,
            }),
            stencil_ops: None,
        };
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Label::from("trivial renderer"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_attachment)
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        self.cylinder_mesh.draw_instanced(&mut render_pass, 2);
    }
}

impl GpuApp for App {
    type UserEvent = UiEvent;

    fn init(&mut self, window: &Window, event_loop: &EventLoop<Self::UserEvent>, _context: &SurfaceContext) {
        #[cfg(target_arch = "wasm32")]
        register_ui_event_handler(&window.canvas(), &event_loop);
    }

    fn on_user_event(&mut self, event: &Self::UserEvent) {
        match event {
            UiEvent::LSystem(LSystemEvent::Iteration(iteration)) => {
                log::info!("Got iteration event: {:?}", iteration);
            }
        }
    }

    fn on_window_event(&mut self, _event: &WindowEvent) {}

    fn render(&mut self, view: &TextureView, input: &Input) {
        let mut command_encoder = self.ctx.device().create_command_encoder(&CommandEncoderDescriptor {
            label: Label::from("frame command encoder"),
        });
        self.render_inner(view, &mut command_encoder);
        self.ctx.queue().submit(vec![command_encoder.finish()]);
    }

    fn get_context_descriptor() -> ContextDescriptor<'static> {
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
