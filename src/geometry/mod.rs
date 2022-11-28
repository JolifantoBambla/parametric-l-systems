use std::mem;
use glam::{Vec2, Vec3, Vec4};
use wgpu::{Buffer, BufferAddress, BufferUsages, Device, IndexFormat, Label, RenderPass, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

const PI_2: f32 = std::f32::consts::PI * 2.0;

pub trait VertexType {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub texture_coordinates: Vec2,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texture_coordinates: Vec2) -> Self {
        Self {
            position,
            normal,
            texture_coordinates,
        }
    }
}

impl VertexType for Vertex {
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
}

pub struct Renderable {
    name: String,
    index_count: u32,
    vertex_count: u32,
    index_buffer: Buffer,
    vertex_buffer: Buffer,
}

impl Renderable {
    pub fn new(name: &String, faces: &Vec<[u32; 3]>, vertices: &Vec<Vertex>, device: &Device) -> Self {
        let indices: Vec<u32> = faces.iter().flatten().cloned().collect();
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Label::from(format!("Index buffer [{}]", name).as_str()),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: BufferUsages::INDEX,
        });
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Label::from(format!("Vertex buffer [{}]", name).as_str()),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: BufferUsages::VERTEX,
        });
        Self {
            name: name.clone(),
            index_count: indices.len() as u32,
            vertex_count: vertices.len() as u32,
            index_buffer,
            vertex_buffer,
        }
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn draw<'a>(&'a self, pass: &mut RenderPass<'a>, num_instances: u32) {
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw_indexed(
            0..self.index_count as u32,
            0,
            0..num_instances
        );
    }
}

pub struct Mesh {
    name: String,
    faces: Vec<[u32; 3]>,
    vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn new_default_cylinder(centered: bool) -> Self {
        Mesh::new_cylinder(8, 4, centered)
    }

    // https://vorg.github.io/pex/docs/pex-gen/Cylinder.html
    pub fn new_cylinder(num_sides: usize, num_segments: usize, centered: bool) -> Self {
        let radius: f32 = 0.5;
        let height: f32 = 1.0;

        // => radius for top & bottom cap
        let r_top = radius;
        let r_bottom = radius;

        // => generate top & bottom cap
        let bottom_cap = true;
        let top_cap = true;

        let mut faces = Vec::new();
        let mut vertices = Vec::new();

        let mut index = 0;

        let offset_y = if centered {
            0.0
        } else {
            - height / 2.
        };

        for j in 0..num_segments + 1 {
            for i in 0..num_sides + 1 {
                let segment_ratio = j as f32 / num_segments as f32;
                let side_ratio = i as f32 / num_sides as f32;
                let r = r_bottom + (r_top - r_bottom) * segment_ratio;
                let y = offset_y + height * segment_ratio;
                let x = r * f32::cos(side_ratio * PI_2);
                let z = r * f32::sin(side_ratio * PI_2);
                vertices.push(Vertex::new(
                    Vec3::new(x, y, z),
                    Vec3::new(x, 0.0, z),
                    Vec2::new(side_ratio, segment_ratio),
                ));
                if i < num_sides && j < num_segments {
                    let i0 = index + 1;
                    let i1 = index;
                    let i2 = index + num_sides as u32 + 1;
                    let i3 = index + num_sides as u32 + 2;
                    faces.push([i0, i1, i2]);
                    faces.push([i0, i2, i3]);
                }
                index += 1;
            }
        }

        if bottom_cap {
            vertices.push(Vertex::new(
                Vec3::new(0.0, offset_y, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec2::new(0.0, 0.0),
            ));
            let center_index = index;
            index += 1;
            for i in 0..num_sides + 1 {
                let y = offset_y;
                let x = r_bottom * f32::cos((i as f32 / num_sides as f32) * PI_2);
                let z = r_bottom * f32::sin((i as f32 / num_sides as f32) * PI_2);
                vertices.push(Vertex::new(
                    Vec3::new(x, y, z),
                    Vec3::new(0.0, -1.0, 0.0),
                    Vec2::new(0.0, 0.0),
                ));
                if i < num_sides {
                    faces.push([index, index + 1, center_index]);
                }
                index += 1;
            }
        }

        if top_cap {
            vertices.push(Vertex::new(
                Vec3::new(0.0, offset_y + height, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec2::new(0.0, 0.0),
            ));
            let center_index = index;
            index += 1;
            for i in 0..num_sides + 1 {
                let y = offset_y + height;
                let x = r_top * f32::cos((i as f32 / num_sides as f32) * PI_2);
                let z = r_top * f32::sin((i as f32 / num_sides as f32) * PI_2);
                vertices.push(Vertex::new(
                    Vec3::new(x, y, z),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec2::new(1.0, 1.0),
                ));
                if i < num_sides {
                    faces.push([index + 1, index, center_index]);
                }
                index += 1;
            }
        }

        Self {
            name: String::from(format!("Cylinder (r={}, h={}, centered={})", radius, height, centered)),
            faces,
            vertices,
        }
    }
}
