use serde::Deserialize;
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
}

impl SceneDescriptor {
    pub fn light_sources(&self) -> &Vec<LightSource> {
        &self.light_sources
    }

    pub fn l_system_settings(&self) -> &Option<LSystemSettings> {
        &self.l_system_settings
    }
}
