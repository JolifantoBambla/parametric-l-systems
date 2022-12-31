use glam::{Mat4, Vec3, Vec4};
use serde::Deserialize;

#[repr(C)]
#[derive(Copy, Clone, Debug, Deserialize, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    albedo: Vec4,
    #[serde(rename = "specularColor")]
    specular_color: Vec3,
    shininess: f32,
}

impl Material {
    pub fn new(albedo: Vec4, specular_color: Vec3, shininess: f32) -> Self {
        Self { albedo, specular_color, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec4::ONE,
            specular_color: Vec3::ZERO,
            shininess: 0.0
        }
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
