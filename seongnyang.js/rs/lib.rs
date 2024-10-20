use wasm_bindgen::prelude::*;

/// Wrapper for dokki
pub fn new_context() -> dokki::CompileContext {
    dokki::new_context()
}

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
