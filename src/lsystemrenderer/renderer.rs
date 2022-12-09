use std::borrow::Cow;
use std::mem;
use std::sync::Arc;
use glam::{Mat4, Vec4};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Color, CompareFunction, DepthStencilState, Extent3d, FragmentState, Label, LoadOp, Operations, PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor, VertexState};
use wgpu::Face::Back;
use crate::framework::camera::Camera;
use crate::framework::context::Gpu;
use crate::framework::gpu::buffer::Buffer;
use crate::framework::mesh::vertex::{Vertex, VertexType};
use crate::framework::renderer::drawable::Draw;
use crate::framework::scene::light::{Light, LightSource, LightSourceType};
use crate::lsystemrenderer::camera::{OrbitCamera, Uniforms};
use crate::lsystemrenderer::scene::LSystemScene;
use crate::lsystemrenderer::turtle::turtle::Instance;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    view: Mat4,
    projection: Mat4,
}

impl From<&OrbitCamera> for CameraUniforms {
    fn from(camera: &OrbitCamera) -> Self {
        Self {
            view: camera.view(),
            projection: camera.projection(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PointLightSourceUniforms {
    light: Light,
    position: Vec4,
}

impl From<&LightSource> for PointLightSourceUniforms {
    fn from(light_source: &LightSource) -> Self {
        Self {
            light: light_source.light(),
            position: match light_source.source() {
                LightSourceType::Directional(d) => {
                    panic!("Can not convert directional light source to point light source uniform");
                }
                LightSourceType::Point(p) => {
                    p.position().extend(1.)
                }
            }
        }
    }
}

pub struct Renderer {
    camera_uniforms: Buffer<CameraUniforms>,
    depth_view: TextureView,
    render_pipeline: RenderPipeline,
    uniforms_bind_group: BindGroup,
    instances_bind_group_layout: Arc<BindGroupLayout>,
}

impl Renderer {
    pub fn new(gpu: &Arc<Gpu>, surface_configuration: &SurfaceConfiguration) -> Self {
        let width = surface_configuration.width;
        let height = surface_configuration.height;

        let depth_format = TextureFormat::Depth32Float;
        let depth_texture = gpu.device().create_texture(&TextureDescriptor {
            label: Label::from("depth texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT,
        });
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let shader_module = gpu.device().create_shader_module(ShaderModuleDescriptor {
            label: Label::from("shader module"),
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let camera_uniforms_bind_group_layout = gpu.device().create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(mem::size_of::<Uniforms>() as _),
                    },
                    count: None,
                }],
            },
        );
        let instances_bind_group_layout = Arc::new(gpu.device().create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Label::from("Instance buffer bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(mem::size_of::<Instance>() as _),
                    },
                    count: None,
                }],
            },
        ));
        let vertex_buffer_layouts = vec![Vertex::buffer_layout()];
        let pipeline_layout = gpu
            .device()
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Label::from("render pipeline layout"),
                bind_group_layouts: &[
                    &camera_uniforms_bind_group_layout,
                    &instances_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let render_pipeline = gpu
            .device()
            .create_render_pipeline(&RenderPipelineDescriptor {
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
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fragment_main",
                    targets: &[Some(surface_configuration.format.into())],
                }),
                multiview: None,
            });

        let camera_uniforms = Buffer::new_zeroed(
            "camera uniforms",
            1,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            gpu,
        );

        let uniforms_bind_group = gpu.device().create_bind_group(&BindGroupDescriptor {
            label: Label::from("uniforms bind group"),
            layout: &camera_uniforms_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_uniforms.buffer().as_entire_binding(),
            }],
        });

        Self {
            camera_uniforms,
            depth_view,
            render_pipeline,
            uniforms_bind_group,
            instances_bind_group_layout,
        }
    }

    pub fn render(&self, render_target: &TextureView, scene: &LSystemScene) {
        self.camera_uniforms
            .write_buffer(&vec![CameraUniforms::from(&scene.camera())]);

        let color_attachment = RenderPassColorAttachment {
            view: render_target,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: true,
            },
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
            depth_stencil_attachment: Some(depth_attachment),
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);

        // todo: get instances from cached bind groups? shouldn't create instance buffer every frame...
        scene.model().draw(&mut render_pass);
    }
}
