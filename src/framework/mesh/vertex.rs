use glam::{Vec2, Vec3};
use std::mem;
use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode};

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
        const VERTEX_ATTRIBUTES: [VertexAttribute; 3] = wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2
        ];
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        }
    }

    fn wgsl_definition<'a>() -> &'a str {
        include_str!("base_vertex.wgsl")
    }
}
