use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use winit::event_loop::EventLoop;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct LSystemIterationEvent {
    iteration: usize,
}

impl LSystemIterationEvent {
    pub fn iteration(&self) -> usize {
        self.iteration
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum LSystemEvent {
    #[serde(rename = "iteration")]
    Iteration(LSystemIterationEvent),
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum UiEvent {
    #[serde(rename = "lSystem")]
    LSystem(LSystemEvent)
}

#[cfg(target_arch = "wasm32")]
pub fn register_ui_event_handler(canvas: &HtmlCanvasElement, event_loop: &EventLoop<UiEvent>) {
    let event_loop_proxy = event_loop.create_proxy();
    let handler: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(move |event| {
        let event: Result<UiEvent, _> = serde_wasm_bindgen::from_value(event);
        match event {
            Ok(event) => {
                match event_loop_proxy.send_event(event) {
                    Ok(_) => {},
                    Err(error) => {
                        log::error!("Could not dispatch event: {}", error);
                    }
                }
            },
            Err(error) => {
                log::error!("Could not process event: {}", error);
            }
        }
    }) as Box<dyn FnMut(JsValue)>);
    canvas
        .add_event_listener_with_callback("ui::lsystem::iteration", handler.as_ref().unchecked_ref())
        .expect("Could not register UI event handler");
    handler.forget();
}
