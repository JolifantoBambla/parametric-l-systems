use crate::framework::camera::Camera;
use crate::framework::context::Gpu;
use crate::framework::event::window::OnResize;
use crate::framework::gpu::buffer::Buffer;
use crate::framework::mesh::vertex::{Vertex, VertexType};
use crate::framework::renderer::drawable::{Draw, DrawInstanced, GpuMesh};
use crate::framework::scene::light::{Light, LightSource, LightSourceType, PointLight};
use crate::lsystemrenderer::camera::OrbitCamera;
use crate::lsystemrenderer::scene::LSystemScene;
use crate::lsystemrenderer::turtle::turtle::Instance;
use glam::{Mat4, Vec3, Vec4};
use std::borrow::Cow;
use std::mem;
use std::sync::Arc;
use wgpu::Face::Back;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Color, CommandEncoder,
    CompareFunction, DepthStencilState, Extent3d, FragmentState, Label, LoadOp, Operations,
    PipelineLayoutDescriptor, PrimitiveState, RenderPass, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
    SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor, VertexState,
};
use crate::framework::scene::transform::Transform;

pub struct RenderObject {
    gpu_mesh: Arc<GpuMesh>,
    num_instances: u32,
    bind_group_index: u32,
    bind_group: BindGroup,
}

impl Draw for RenderObject {
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_bind_group(self.bind_group_index, &self.bind_group, &[]);
        self.gpu_mesh.draw_instanced(pass, self.num_instances);
    }
}

pub struct RenderObjectCreator {
    gpu: Arc<Gpu>,
    bind_group_index: u32,
    bind_group_layout: BindGroupLayout,
}

impl RenderObjectCreator {
    pub fn create_render_object(
        &self,
        mesh: &Arc<GpuMesh>,
        transform: &Buffer<Mat4>,
        instances: &Buffer<Instance>,
    ) -> RenderObject {
        let bind_group = self.gpu.device().create_bind_group(&BindGroupDescriptor {
            label: Label::from("instances bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: instances.buffer().as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: transform.buffer().as_entire_binding(),
                }
            ],
        });
        RenderObject {
            gpu_mesh: mesh.clone(),
            num_instances: instances.num_elements() as u32,
            bind_group_index: self.bind_group_index,
            bind_group,
        }
    }
}

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
struct LightSourceUniforms {
    light: Light,
    position_or_direction: Vec3,
    light_type: u32,
}

impl From<&LightSource> for LightSourceUniforms {
    fn from(light_source: &LightSource) -> Self {
        Self {
            light: light_source.light(),
            position_or_direction: match light_source.source() {
                LightSourceType::Directional(d) => d.direction(),
                LightSourceType::Point(p) => p.position(),
                LightSourceType::Ambient => {
                    panic!("Ambient light source does not have a position or direction");
                },
            },
            light_type: match light_source.source() {
                LightSourceType::Directional(_) => 0,
                LightSourceType::Point(_) => 1,
                LightSourceType::Ambient => {
                    panic!("Ambient light source can not be mapped to a LightSourceUniform");
                },
            },
        }
    }
}

pub struct LightSourcesBindGroup {
    bind_group_index: u32,
    bind_group: BindGroup,
}

impl LightSourcesBindGroup {
    pub fn set_bind_group<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_bind_group(self.bind_group_index, &self.bind_group, &[]);
    }
}

pub struct LightSourcesBindGroupCreator {
    gpu: Arc<Gpu>,
    bind_group_index: u32,
    bind_group_layout: BindGroupLayout,
}

impl LightSourcesBindGroupCreator {
    pub fn create(&self, lights: &Vec<LightSource>) -> LightSourcesBindGroup {
        let lights_buffer: Buffer<LightSourceUniforms> = Buffer::from_data(
            "light sources buffer",
            &lights.iter().map(|l| l.into()).collect(),
            BufferUsages::STORAGE,
            &self.gpu
        );
        let bind_group = self.gpu.device().create_bind_group(&BindGroupDescriptor {
            label: Label::from("light sources bind group"),
            layout: &self.bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: lights_buffer.buffer().as_entire_binding(),
            }],
        });
        LightSourcesBindGroup {
            bind_group_index: self.bind_group_index,
            bind_group,
        }
    }
}

pub struct Renderer {
    camera_uniforms: Buffer<CameraUniforms>,
    depth_view: TextureView,
    render_pipeline: RenderPipeline,
    uniforms_bind_group: BindGroup,
    render_object_creator: RenderObjectCreator,
    light_sources_bind_group_creator: LightSourcesBindGroupCreator,
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

        let camera_uniforms_bind_group_layout =
            gpu.device()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                mem::size_of::<CameraUniforms>() as _,
                            ),
                        },
                        count: None,
                    }],
                });
        let instances_bind_group_layout = gpu.device().create_bind_group_layout(
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
        );
        let light_sources_bind_group_layout =
            gpu.device()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Label::from("Light sources bind group layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(
                                mem::size_of::<LightSourceUniforms>() as _
                            ),
                        },
                        count: None,
                    }],
                });
        let vertex_buffer_layouts = vec![Vertex::buffer_layout()];
        let pipeline_layout = gpu
            .device()
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Label::from("render pipeline layout"),
                bind_group_layouts: &[
                    &camera_uniforms_bind_group_layout,
                    &instances_bind_group_layout,
                    &light_sources_bind_group_layout,
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
            render_object_creator: RenderObjectCreator {
                gpu: gpu.clone(),
                bind_group_index: 1,
                bind_group_layout: instances_bind_group_layout,
            },
            light_sources_bind_group_creator: LightSourcesBindGroupCreator {
                gpu: gpu.clone(),
                bind_group_index: 2,
                bind_group_layout: light_sources_bind_group_layout,
            }
        }
    }

    pub fn render(
        &self,
        render_target: &TextureView,
        scene: &LSystemScene,
        command_encoder: &mut CommandEncoder,
    ) {
        let background_color = scene.background_color();

        self.camera_uniforms
            .write_buffer(&vec![CameraUniforms::from(&scene.camera())]);

        let color_attachment = RenderPassColorAttachment {
            view: render_target,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {
                    r: background_color.x as f64,
                    g: background_color.y as f64,
                    b: background_color.z as f64,
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

        scene.get_light_sources_bind_group().set_bind_group(&mut render_pass);

        for render_objects in scene.get_active_render_objects() {
            for render_object in render_objects {
                render_object.draw(&mut render_pass);
            }
        }
    }
    pub fn render_object_creator(&self) -> &RenderObjectCreator {
        &self.render_object_creator
    }
    pub fn light_sources_bind_group_creator(&self) -> &LightSourcesBindGroupCreator {
        &self.light_sources_bind_group_creator
    }
}

impl OnResize for Renderer {
    fn on_resize(&mut self, width: u32, height: u32) {
        // todo: recreate depth texture
        log::error!("resize not implemented for renderer yet");
    }
}
