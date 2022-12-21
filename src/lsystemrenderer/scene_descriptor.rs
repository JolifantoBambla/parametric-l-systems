use std::collections::HashMap;
use glam::{Mat4, Vec3};
use serde::Deserialize;
use wasm_bindgen::JsValue;
use crate::framework::scene::transform::Transform;
use crate::LightSource;
use crate::lsystemrenderer::turtle::turtle::Material;

#[derive(Clone, Debug, Deserialize)]
pub struct LSystemSettings {
    #[serde(rename = "startMaterial")]
    start_material: Option<usize>,

    materials: Option<Vec<Material>>,
}

impl LSystemSettings {
    pub fn start_material(&self) -> usize {
        self.start_material.unwrap_or(0)
    }
    pub fn materials(&self) -> &Option<Vec<Material>> {
        &self.materials
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SceneDescriptor {
    #[serde(rename = "lightSources")]
    light_sources: Vec<LightSource>,

    #[serde(rename = "lSystemSettings")]
    l_system_settings: Option<LSystemSettings>,

    transform: Transform,
}

impl SceneDescriptor {
    pub fn light_sources(&self) -> &Vec<LightSource> {
        &self.light_sources
    }

    pub fn l_system_settings(&self) -> &Option<LSystemSettings> {
        &self.l_system_settings
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}


#[derive(Clone, Debug, Deserialize)]
struct LSystemInstance {
    //#[serde(rename = "lSystem")]
    //l_system: JsValue,
    transform: Transform,
    #[serde(rename = "lSystemSettings")]
    materials: Option<LSystemSettings>,
}

#[derive(Clone, Debug, Deserialize)]
struct CameraDescriptor {
    eye: Vec3,
    #[serde(rename = "lookAt")]
    look_at: Vec3,
    up: Vec3,
}

#[derive(Clone, Debug, Deserialize)]
struct Light {
    color: Vec3,
}

#[derive(Clone, Debug, Deserialize)]
struct LightsDescriptor {
    ambient: Light,

}

struct Scene {

}

struct Foo {
    systems: HashMap<String, HashMap<String, LSystemInstance>>,
    scene: Scene,
}

