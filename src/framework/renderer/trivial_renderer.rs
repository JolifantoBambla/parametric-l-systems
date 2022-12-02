// todo: refactor / remove

use std::rc::Rc;
use glam::{Mat4, Vec2};
use wgpu::{BufferUsages, Color, CommandEncoder, Extent3d, Label, LoadOp, Operations, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor};
use crate::framework::camera::Camera;
use crate::framework::context::{Gpu, SurfaceContext};
use crate::framework::gpu::buffer::Buffer;
use crate::framework::input::Input;
use crate::framework::renderer::Renderer;
use crate::framework::scene::Scene;

#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    view: Mat4,
    projection: Mat4,
    frame: u32,
    delta: f32,
    _padding: Vec2,
}

pub struct TrivialRenderer {
    uniforms: Buffer<Uniforms>,
    depth_view: TextureView,
}

impl TrivialRenderer {
    pub fn new(width: u32, height: u32, ctx: &Rc<Gpu>) -> Self {
        let uniform_buffer = Buffer::new_single_element(
            "uniforms",
            Uniforms::default(),
            BufferUsages::UNIFORM,
            ctx,
        );

        let depth_texture = ctx.device().create_texture(&TextureDescriptor {
            label: Label::from("depth texture"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth24Plus,
            usage: TextureUsages::RENDER_ATTACHMENT,
        });
        // todo: maybe this needs to be explicitly set
        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        Self {
            uniforms: uniform_buffer,
            depth_view,
        }
    }

    fn update_uniforms(&self, uniforms: Uniforms) {
        self.uniforms.write_buffer(&vec![uniforms]);
    }
}

impl Renderer for TrivialRenderer {
    fn render(&self, render_target: &TextureView, camera: &Box<dyn Camera>, input: &Input, scene: &Scene, command_encoder: &mut CommandEncoder) {
        self.update_uniforms(Uniforms {
            view: camera.view(),
            projection: camera.projection(),
            frame: input.frame().number(),
            delta: input.time().delta(),
            _padding: Default::default(),
        });

        let color_attachment = RenderPassColorAttachment {
            view: render_target,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0}),
                store: false,
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

        todo!("get drawables from scene and draw")
    }
}