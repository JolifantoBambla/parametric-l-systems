use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{CustomEvent, HtmlCanvasElement};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;

pub trait OnResize {
    fn on_resize(&mut self, width: u32, height: u32);
}

pub trait OnWindowEvent {
    fn on_window_event(&mut self, event: &WindowEvent);
}

pub trait OnUserEvent {
    type UserEvent;

    fn on_user_event(&mut self, event: &Self::UserEvent);
}

#[cfg(target_arch = "wasm32")]
pub fn register_custom_canvas_event_dispatcher<T: for<'de> Deserialize<'de>>(event_name: &str, canvas: &HtmlCanvasElement, event_loop: &EventLoop<T>) {
    let event_loop_proxy = event_loop.create_proxy();
    let handler: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(move |canvas_event: JsValue| {
        match canvas_event.dyn_into::<CustomEvent>() {
            Ok(custom_event) => {
                let event: Result<T, _> = serde_wasm_bindgen::from_value(custom_event.detail());
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
            },
            Err(error) => {
                log::error!("Could not cast JsValue to web_sys::CustomEvent: {:?}", error);
            }
        }
    }) as Box<dyn FnMut(JsValue)>);
    canvas
        .add_event_listener_with_callback(event_name, handler.as_ref().unchecked_ref())
        .expect("Could not register canvas event handler");
    handler.forget();
}
