use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/js/l-system/l-system.js")]
extern "C" {
    type LSystemIterator;

    #[wasm_bindgen(static_method_of = LSystemIterator, js_name = "createLSystemIterator")]
    fn create_l_system_iterator(definition: JsValue) -> LSystemIterator;

    #[wasm_bindgen(method, js_name = "current")]
    fn current(this: &LSystemIterator, as_string: bool) -> JsValue;

    #[wasm_bindgen(method, js_name = "next")]
    fn next(this: &LSystemIterator, as_string: bool) -> JsValue;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LSystemParameterValue {
    String(String),
    Float(f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LSystemDefinition {
    alphabet: Vec<String>,
    parameters: HashMap<String, LSystemParameterValue>,
    productions: Vec<String>,
    axiom: String,
}

pub struct LSystem {
    #[cfg(target_arch = "wasm32")]
    l_system_iterator: LSystemIterator,
}

impl LSystem {
    #[cfg(target_arch = "wasm32")]
    pub fn new(definition: &LSystemDefinition) -> Self {
        Self {
            l_system_iterator: LSystemIterator::create_l_system_iterator(
                serde_wasm_bindgen::to_value(definition)
                    .ok()
                    .expect("Could not serialize LSystemDefinition")
            ),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn next_raw(&self) -> JsValue {
        self.l_system_iterator.next(false)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn next(&self) -> String {
        self.l_system_iterator
            .next(true)
            .as_string()
            .expect("L-System state was no String")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn current_raw(&self) -> JsValue {
        self.l_system_iterator.current(false)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn current(&self) -> String {
        self.l_system_iterator
            .current(true)
            .as_string()
            .expect("L-System state was no String")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_js_systems(js_system_definitions: JsValue) -> HashMap<String, HashMap<String, LSystem>> {
        let l_system_definitions: HashMap<String, HashMap<String, LSystemDefinition>> = serde_wasm_bindgen::from_value(js_system_definitions)
            .expect("Could not deserialize LSystem definitions");
        LSystem::from_l_system_definitions(&l_system_definitions)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_l_system_definitions(definitions: &HashMap<String, HashMap<String, LSystemDefinition>>) -> HashMap<String, HashMap<String, LSystem>> {
        let mut l_systems = HashMap::new();
        for (system_name, system) in definitions {
            let mut instances = HashMap::new();
            for (instance_name, instance) in system {
                instances.insert(instance_name.clone(), LSystem::new(&instance));
            }
            l_systems.insert(system_name.clone(), instances);
        }
        l_systems
    }
}

#[cfg(target_arch = "wasm32")]
impl From<JsValue> for LSystem {
    fn from(definition: JsValue) -> Self {
        Self {
            l_system_iterator: LSystemIterator::create_l_system_iterator(definition),
        }
    }
}
