use wasm_bindgen::prelude::*;

pub mod framework;
pub mod lindenmayer;
pub mod lsystemrenderer;

use crate::framework::app::AppRunner;
use crate::framework::scene::light::LightSource;
use crate::framework::util::window::WindowConfig;
use crate::lsystemrenderer::App;
use crate::lsystemrenderer::scene_descriptor::SceneDescriptor;

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
pub fn main(scene: JsValue, l_system_definition: JsValue) {
    let scene_descriptor: SceneDescriptor = serde_wasm_bindgen::from_value(scene)
        .expect("Could not deserialize scene descriptor");
    let l_system = lindenmayer::LSystem::new(l_system_definition);
    wasm_bindgen_futures::spawn_local(run(scene_descriptor, l_system));
}

#[cfg(target_arch = "wasm32")]
async fn run(scene_descriptor: SceneDescriptor, l_system: lindenmayer::LSystem) {
    let window_config = WindowConfig::default();
    let app_runner = AppRunner::<App>::new(window_config).await;
    let app = App::new(
        app_runner.ctx().gpu(),
        app_runner.ctx().surface_configuration(),
        l_system,
        scene_descriptor,
    );
    app_runner.run(app);
}
