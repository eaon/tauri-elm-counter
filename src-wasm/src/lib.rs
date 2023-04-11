use wasm_bindgen::prelude::*;
use js_sys::Number;

#[wasm_bindgen]
pub fn decrement(current: Number) -> Number {
    (current.value_of() - 1f64).into()
}
