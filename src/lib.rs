use wasm_bindgen::prelude::*;

pub mod turtle;
pub mod lindenmayer;
pub mod framework;
pub mod lsystemrenderer;

use crate::lsystemrenderer::App;
use crate::framework::app::AppRunner;
use crate::framework::util::window::WindowConfig;


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
pub fn main(l_system_definition: JsValue) {
    let l_system = lindenmayer::LSystem::new(l_system_definition);
    for i in 0..5 {
        let commands: Vec<turtle::TurtleCommand> = serde_wasm_bindgen::from_value(l_system.next_raw())
            .expect("Could not parse turtle commands");
        turtle::execute_turtle_commands(&commands);
    }
    wasm_bindgen_futures::spawn_local(run());
}

#[cfg(target_arch = "wasm32")]
async fn run() {
    let window_config = WindowConfig::default();
    let app_runner = AppRunner::<App>::new(window_config)
        .await;
    let app = App::new(&app_runner.ctx().gpu(), &app_runner.ctx().surface_configuration());
    app_runner.run(app);
}
