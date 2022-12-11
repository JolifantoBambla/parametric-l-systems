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

pub struct LSystem {
    #[cfg(target_arch = "wasm32")]
    l_system_iterator: LSystemIterator,
}

impl LSystem {
    #[cfg(target_arch = "wasm32")]
    pub fn new(definition: JsValue) -> Self {
        Self {
            l_system_iterator: LSystemIterator::create_l_system_iterator(definition),
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
}
