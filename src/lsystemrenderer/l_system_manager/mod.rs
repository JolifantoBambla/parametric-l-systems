use crate::framework::context::Gpu;
use crate::framework::event::lifecycle::Update;
use crate::framework::input::Input;
use crate::framework::scene::transform::Transform;
use crate::lsystemrenderer::l_system_manager::command::TurtleCommand;
use crate::lsystemrenderer::l_system_manager::turtle::{
    LSystemModel, LSystemPrimitive, MaterialState, Tropism,
};
use crate::LSystem;
use std::collections::HashMap;
use std::sync::Arc;

pub mod command;
pub mod turtle;

pub struct LSystemManager {
    gpu: Arc<Gpu>,
    max_time_to_iterate: f32,
    transform: Transform,
    l_system: LSystem,
    max_target_iteration: u32,
    iterations: Vec<LSystemModel>,
    material_state: MaterialState,
    primitives: HashMap<String, LSystemPrimitive>,
    tropism: Option<Tropism>,
}

impl LSystemManager {
    pub fn new(
        l_system: LSystem,
        transform: Transform,
        initial_material_state: Option<MaterialState>,
        primitives: HashMap<String, LSystemPrimitive>,
        tropism: Option<Tropism>,
        gpu: &Arc<Gpu>,
    ) -> Self {
        let mut iterations = Vec::new();
        let commands: Vec<TurtleCommand> = serde_wasm_bindgen::from_value(l_system.next_raw())
            .expect("Could not parse turtle commands");

        let material_state = initial_material_state.unwrap_or_default();

        // evaluate the first iteration
        iterations.push(LSystemModel::from_turtle_commands(
            &commands,
            transform,
            material_state.clone(),
            &primitives,
            &tropism,
            gpu,
        ));

        Self {
            gpu: gpu.clone(),
            max_time_to_iterate: 50.,
            transform,
            l_system,
            max_target_iteration: 0,
            iterations,
            material_state,
            primitives,
            tropism,
        }
    }

    pub fn maybe_increase_max_iteration(&mut self, max_iteration: u32) {
        self.max_target_iteration = max_iteration.max(self.max_target_iteration);
    }

    pub fn try_get_iteration(&self, iteration: u32) -> (u32, &LSystemModel) {
        if self.iterations.len() as u32 > iteration {
            (iteration, &self.iterations[iteration as usize])
        } else {
            let i = self.iterations.len() - 1;
            (i as u32, &self.iterations[i])
        }
    }
}

impl Update for LSystemManager {
    fn update(&mut self, input: &Input) {
        // if iterations have been requested but not yet evaluated, evaluate more iterations until
        // either all requested iterations have been evaluated, or the time to iterate per frame is
        // exceeded
        while self.max_target_iteration >= self.iterations.len() as u32 {
            let commands: Vec<TurtleCommand> =
                serde_wasm_bindgen::from_value(self.l_system.next_raw())
                    .expect("Could not parse turtle commands");
            self.iterations.push(LSystemModel::from_turtle_commands(
                &commands,
                self.transform,
                self.material_state.clone(),
                &self.primitives,
                &self.tropism,
                &self.gpu,
            ));
            if instant::now() as f32 - input.time().now() >= self.max_time_to_iterate {
                break;
            }
        }
    }
}

impl Drop for LSystemManager {
    fn drop(&mut self) {
        for iteration in self.iterations.iter_mut() {
            iteration.cylinder_instances_buffer().buffer().destroy();
        }
    }
}
