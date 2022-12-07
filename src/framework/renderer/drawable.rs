use js_sys::ReferenceError;
use wgpu::{Buffer, BufferUsages, Device, IndexFormat, Label, RenderPass};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::framework::mesh::{mesh::Mesh, vertex::VertexType};

pub trait DrawInstanced {
    fn draw_instanced<'a>(&'a self, pass: &mut RenderPass<'a>, num_instances: u32);
}

pub trait Draw {
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>);
}

pub struct GpuMesh {
    name: String,
    index_count: u32,
    vertex_count: u32,
    index_buffer: Buffer,
    vertex_buffer: Buffer,
}

impl GpuMesh {
    pub fn new<V: VertexType>(name: &String, faces: &Vec<[u32; 3]>, vertices: &Vec<V>, device: &Device) -> Self {
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

    pub fn from_mesh<V: VertexType>(mesh: &Mesh<V>, device: &Device) -> Self {
        GpuMesh::new(&String::from(mesh.name()), mesh.faces(), mesh.vertices(), device)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }
}

impl DrawInstanced for GpuMesh {
    fn draw_instanced<'a>(&'a self, pass: &mut RenderPass<'a>, num_instances: u32) {
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw_indexed(
            0..self.index_count as u32,
            0,
            0..num_instances
        );
    }
}

impl Draw for GpuMesh {
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>) {
        self.draw_instanced(pass, 1);
    }
}
