use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use winit::event_loop::EventLoop;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum LSystemEvent {
    #[serde(rename = "iteration")]
    Iteration(usize),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum UiEvent {
    #[serde(rename = "lSystem")]
    LSystem(LSystemEvent)
}
