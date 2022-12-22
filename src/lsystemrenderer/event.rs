use glam::Vec3;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct IterationEvent {
    #[serde(rename = "objectName")]
    object_name: String,
    iteration: u32,
}

impl IterationEvent {
    pub fn object_name(&self) -> &str {
        &self.object_name
    }
    pub fn iteration(&self) -> u32 {
        self.iteration
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum LSystemEvent {
    #[serde(rename = "iteration")]
    Iteration(IterationEvent),
}

#[derive(Clone, Debug, Deserialize)]
pub enum SceneEvent {
    #[serde(rename = "backgroundColor")]
    BackgroundColor(Vec3),
}

#[derive(Clone, Debug, Deserialize)]
pub enum UiEvent {
    #[serde(rename = "lSystem")]
    LSystem(LSystemEvent),
    #[serde(rename = "scene")]
    Scene(SceneEvent,)
}
