use std::path::PathBuf;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{
    Document, Element, HtmlCanvasElement, HtmlElement, HtmlInputElement, Performance, Window,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::WindowBuilder,
};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::{WindowBuilderExtWebSys, WindowExtWebSys};

#[inline]
pub fn window() -> Window {
    web_sys::window().unwrap_or_else(|| panic!("window does not exist"))
}

#[inline]
pub fn document() -> Document {
    window()
        .document()
        .unwrap_or_else(|| panic!("window has no document"))
}

#[inline]
pub fn body() -> HtmlElement {
    document()
        .body()
        .unwrap_or_else(|| panic!("document has no body"))
}

#[inline]
pub fn url() -> String {
    document()
        .url()
        .unwrap_or_else(|_| panic!("document has no URL"))
}

#[inline]
pub fn get_element_by_id(id: &str) -> Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("document has no element with id {}", id))
}

#[inline]
pub fn get_input_element_by_id(id: &str) -> HtmlInputElement {
    get_element_by_id(id)
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| ())
        .unwrap_or_else(|_| panic!("element with id {} was no input element", id))
}

#[inline]
pub fn get_canvas_by_id(id: &str) -> HtmlCanvasElement {
    get_element_by_id(id)
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap_or_else(|_| panic!("element with id {} was no canvas", id))
}

/// Attaches a given HtmlCanvasElement to the document.
/// If a parent_id is given, the canvas is appended as child of the parent element.
/// Otherwise, the canvas as attached to the body of the document.
#[inline]
pub fn attach_canvas(canvas: HtmlCanvasElement, parent_id: &Option<String>) {
    let parent = if let Some(parent_id) = parent_id {
        get_element_by_id(parent_id.as_str())
            .dyn_into::<HtmlElement>()
            .map_err(|_| ())
            .unwrap()
    } else {
        body().dyn_into::<HtmlElement>().map_err(|_| ()).unwrap()
    };
    parent
        .append_child(&web_sys::Element::from(canvas))
        .unwrap_or_else(|_| panic!("could not append element to document"));
}

#[inline]
pub fn base_path() -> PathBuf {
    let base_url = url();
    if !base_url.ends_with('/') {
        PathBuf::from(base_url).parent().unwrap().to_path_buf()
    } else {
        PathBuf::from(base_url)
    }
}

pub struct WindowConfig {
    title: String,
    size: PhysicalSize<u32>,
    canvas_id: Option<String>,
    parent_id: Option<String>,
}

impl WindowConfig {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
    pub fn canvas_id(&self) -> &Option<String> {
        &self.canvas_id
    }
    pub fn parent_id(&self) -> &Option<String> {
        &self.parent_id
    }
}

pub fn get_or_create_window(window_config: &WindowConfig, event_loop: &EventLoop<()>) -> window::Window {
    let mut builder = WindowBuilder::new()
        .with_title(window_config.title())
        .with_inner_size(window_config.size());
    if let Some(canvas_id) = window_config.canvas_id() {
        let canvas = get_canvas_by_id(canvas_id.as_str());
        let canvas_size = PhysicalSize {
            width: canvas.width(),
            height: canvas.height(),
        };
        builder = builder
            .with_canvas(Some(canvas))
            .with_inner_size(canvas_size);
    }

    let window = builder.build(&event_loop).unwrap();

    if window_config.canvas_id.is_none() {
        attach_canvas(window.canvas(), window_config.parent_id());
    }

    window
}