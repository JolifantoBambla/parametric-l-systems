use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum LSystemEvent {
    #[serde(rename = "iteration")]
    Iteration(usize),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum UiEvent {
    #[serde(rename = "lSystem")]
    LSystem(LSystemEvent),
}
