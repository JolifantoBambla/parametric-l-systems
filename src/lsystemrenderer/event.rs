use crate::lindenmayer::LSystemDefinition;
use crate::LSystemSceneDescriptor;
use glam::Vec3;
use serde::Deserialize;
use std::collections::HashMap;

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
pub struct NewSceneEvent {
    #[serde(rename = "sceneDescriptor")]
    scene_descriptor: LSystemSceneDescriptor,
    #[serde(rename = "lSystemDefinitions")]
    l_system_definitions: HashMap<String, HashMap<String, LSystemDefinition>>,
}

impl NewSceneEvent {
    pub fn scene_descriptor(&self) -> &LSystemSceneDescriptor {
        &self.scene_descriptor
    }
    pub fn l_system_definitions(&self) -> &HashMap<String, HashMap<String, LSystemDefinition>> {
        &self.l_system_definitions
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum SceneEvent {
    #[serde(rename = "backgroundColor")]
    BackgroundColor(Vec3),

    #[serde(rename = "new")]
    New(Box<NewSceneEvent>),
}

#[derive(Clone, Debug, Deserialize)]
pub enum UiEvent {
    #[serde(rename = "lSystem")]
    LSystem(LSystemEvent),
    #[serde(rename = "scene")]
    Scene(SceneEvent),
}
