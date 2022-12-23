use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub mod framework;
pub mod lindenmayer;
pub mod lsystemrenderer;

use crate::framework::app::AppRunner;
use crate::framework::util::window::WindowConfig;
use crate::lindenmayer::LSystem;
use crate::lsystemrenderer::scene_descriptor::LSystemSceneDescriptor;
use crate::lsystemrenderer::App;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn initialize() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen()]
pub fn main(canvas_id: String, scene: JsValue, l_system_definitions: JsValue) {
    let scene_descriptor: LSystemSceneDescriptor =
        serde_wasm_bindgen::from_value(scene).expect("Could not deserialize scene descriptor");
    let l_systems = LSystem::from_js_systems(l_system_definitions);
    wasm_bindgen_futures::spawn_local(run(canvas_id, scene_descriptor, l_systems));
}

#[cfg(target_arch = "wasm32")]
async fn run(
    canvas_id: String,
    scene_descriptor: LSystemSceneDescriptor,
    l_systems: HashMap<String, HashMap<String, LSystem>>,
) {
    let window_config = WindowConfig::new_with_canvas("L-System Viewer".to_string(), canvas_id);
    let app_runner = AppRunner::<App>::new(window_config).await;
    let app = App::new(
        app_runner.ctx().gpu(),
        app_runner.ctx().surface_configuration(),
        l_systems,
        scene_descriptor,
    );
    app_runner.run(app);
}
