use glam::{Mat4, Vec4};
use serde::Deserialize;

#[repr(C)]
#[derive(Copy, Clone, Debug, Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    color: Vec4,
}

impl Material {
    pub fn new(color: Vec4) -> Self {
        Self { color }
    }
    pub fn color(&self) -> Vec4 {
        self.color
    }
    pub fn set_color(&mut self, color: Vec4) {
        self.color = color;
    }
}

impl Default for Material {
    fn default() -> Self {
        Self { color: Vec4::ONE }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    matrix: Mat4,
    material: Material,
}

impl Instance {
    pub fn new(matrix: Mat4, material: Material) -> Self {
        Self { matrix, material }
    }
    pub fn matrix(&self) -> Mat4 {
        self.matrix
    }
    pub fn set_matrix(&mut self, matrix: Mat4) {
        self.matrix = matrix;
    }
    pub fn material(&self) -> Material {
        self.material
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
            material: Material::default(),
        }
    }
}
