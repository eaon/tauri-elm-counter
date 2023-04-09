use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decrement(current: i64) -> i64 {
    current - 1
}
