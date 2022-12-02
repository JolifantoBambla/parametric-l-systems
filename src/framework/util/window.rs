use winit::dpi::PhysicalSize;

pub trait Resize {
    fn resize(&mut self, width: u32, height: u32);
}

pub struct WindowConfig {
    title: String,
    size: PhysicalSize<u32>,

    #[cfg(target_arch = "wasm32")]
    canvas_id: Option<String>,

    #[cfg(target_arch = "wasm32")]
    parent_id: Option<String>,
}

impl WindowConfig {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    #[cfg(target_arch = "wasm32")]
    pub fn canvas_id(&self) -> &Option<String> {
        &self.canvas_id
    }

    #[cfg(target_arch = "wasm32")]
    pub fn parent_id(&self) -> &Option<String> {
        &self.parent_id
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            size: PhysicalSize {
                width: 800,
                height: 600,
            },
            #[cfg(target_arch = "wasm32")]
            canvas_id: None,
            #[cfg(target_arch = "wasm32")]
            parent_id: None,
        }
    }
}