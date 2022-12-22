use std::collections::HashMap;
use glam::Vec3;
use serde::Deserialize;
use crate::framework::camera::CameraView;
use crate::framework::scene::transform::Transform;
use crate::lsystemrenderer::turtle::turtle::{Material, MaterialState};

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemInstance {
    iterations: u32,

    transform: Option<Transform>,

    #[serde(rename = "startMaterial")]
    start_material: Option<usize>,

    materials: Option<Vec<Material>>,
}

impl LSystemInstance {
    pub fn iterations(&self) -> u32 {
        self.iterations
    }
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or(Transform::default())
    }
    pub fn start_material(&self) -> usize {
        self.start_material.unwrap_or(0)
    }
    pub fn materials(&self) -> &Option<Vec<Material>> {
        &self.materials
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemDescriptor {
    #[serde(rename = "type")]
    system_type: String,
    instances: HashMap<String, LSystemInstance>,
    transform: Option<Transform>,
}

impl LSystemDescriptor {
    pub fn system_type(&self) -> &String {
        &self.system_type
    }
    pub fn instances(&self) -> &HashMap<String, LSystemInstance> {
        &self.instances
    }
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or(Transform::default())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CameraDescriptor {
    eye: Vec3,

    #[serde(rename = "lookAt")]
    look_at: Vec3,

    up: Vec3,

    #[serde(rename = "backgroundColor")]
    background_color: Option<Vec3>,
}

impl CameraDescriptor {
    pub fn eye(&self) -> Vec3 {
        self.eye
    }
    pub fn look_at(&self) -> Vec3 {
        self.look_at
    }
    pub fn up(&self) -> Vec3 {
        self.up
    }
    pub fn background_color(&self) -> Vec3 {
        self.background_color.unwrap_or(Vec3::ZERO)
    }
}

impl From<&CameraDescriptor> for CameraView {
    fn from(descriptor: &CameraDescriptor) -> Self {
        CameraView::new(descriptor.eye(), descriptor.look_at(), descriptor.up())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AmbientLightDescriptor {
    color: Vec3,
}

impl AmbientLightDescriptor {
    pub fn color(&self) -> Vec3 {
        self.color
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PointLightDescriptor {
    color: Vec3,
    position: Vec3,
}

impl PointLightDescriptor {
    pub fn color(&self) -> Vec3 {
        self.color
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DirectionalLightDescriptor {
    color: Vec3,
    direction: Vec3,
}

impl DirectionalLightDescriptor {
    pub fn color(&self) -> Vec3 {
        self.color
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct LightsDescriptor {
    ambient: Option<AmbientLightDescriptor>,

    #[serde(rename = "pointLights")]
    point_lights: Vec<PointLightDescriptor>,

    #[serde(rename = "directionalLights")]
    directional_lights: Vec<DirectionalLightDescriptor>,
}

impl LightsDescriptor {
    pub fn ambient(&self) -> &Option<AmbientLightDescriptor> {
        &self.ambient
    }
    pub fn point_lights(&self) -> &Vec<PointLightDescriptor> {
        &self.point_lights
    }
    pub fn directional_lights(&self) -> &Vec<DirectionalLightDescriptor> {
        &self.directional_lights
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemObjectDescriptor {
    transform: Option<Transform>,
    system: String,
    instance: String,
    iteration: Option<u32>,
}

impl LSystemObjectDescriptor {
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or(Transform::default())
    }
    pub fn system(&self) -> &str {
        &self.system
    }
    pub fn instance(&self) -> &str {
        &self.instance
    }
    pub fn iteration(&self) -> &Option<u32> {
        &self.iteration
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ObjObject {
    transform: Option<Transform>,
    obj: String,
    material: Material,
}

impl ObjObject {
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or(Transform::default())
    }
    pub fn obj(&self) -> &str {
        &self.obj
    }
    pub fn material(&self) -> Material {
        self.material
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SceneObjectDescriptor {
    #[serde(rename = "lSystem")]
    LSystem(LSystemObjectDescriptor),

    #[serde(rename = "obj")]
    Obj(ObjObject),
}

#[derive(Clone, Debug, Deserialize)]
pub struct ObjResource {
    source: String,
    transform: Option<Transform>,
}

impl ObjResource {
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn transform(&self) -> Transform {
        self.transform.unwrap_or(Transform::default())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SceneResource {
    #[serde(rename = "obj")]
    Obj(ObjResource)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    camera: CameraDescriptor,
    lights: LightsDescriptor,
    objects: HashMap<String, SceneObjectDescriptor>,
}

impl Scene {
    pub fn camera(&self) -> &CameraDescriptor {
        &self.camera
    }
    pub fn lights(&self) -> &LightsDescriptor {
        &self.lights
    }
    pub fn objects(&self) -> &HashMap<String, SceneObjectDescriptor> {
        &self.objects
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemSceneDescriptor {
    systems: HashMap<String, LSystemDescriptor>,
    scene: Scene,
    resources: Option<HashMap<String, SceneResource>>,
}

impl LSystemSceneDescriptor {
    pub fn systems(&self) -> &HashMap<String, LSystemDescriptor> {
        &self.systems
    }
    pub fn scene(&self) -> &Scene {
        &self.scene
    }
    pub fn resources(&self) -> &Option<HashMap<String, SceneResource>> {
        &self.resources
    }
}
