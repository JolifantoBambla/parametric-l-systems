use std::mem;
use glam::{Vec2, Vec3};
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub trait VertexType: bytemuck::Pod {
    fn from_pos_normal_tex_coords(position: Vec3, normal: Vec3, texture_coordinates: Vec2) -> Self;
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn wgsl_definition<'a>() -> &'a str;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    texture_coordinates: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texture_coordinates: Vec2) -> Self {
        Self {
            position,
            normal,
            texture_coordinates,
        }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn texture_coordinates(&self) -> Vec2 {
        self.texture_coordinates
    }
}

impl VertexType for Vertex {
    fn from_pos_normal_tex_coords(position: Vec3, normal: Vec3, texture_coordinates: Vec2) -> Self {
        Vertex::new(position, normal, texture_coordinates)
    }

    fn buffer_layout<'a>() -> VertexBufferLayout<'a> {
        const SIZE_VEC3: usize = mem::size_of::<[f32; 3]>();
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: SIZE_VEC3 as BufferAddress,
                    shader_location: 1
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: (SIZE_VEC3 * 2) as BufferAddress,
                    shader_location: 2
                }
            ]
        }
    }

    fn wgsl_definition<'a>() -> &'a str {
        include_str!("base_vertex.wgsl")
    }
}
