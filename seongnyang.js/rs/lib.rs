use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct Context {
    ctx: dokki::CompileContext,
}

/// Wrapper for dokki
#[wasm_bindgen]
#[allow(private_interfaces)]
pub fn new_context() -> Context {
    Context {
        ctx: dokki::new_context(),
    }
}

#[wasm_bindgen]
#[allow(private_interfaces)]
pub fn compile(context: Context, source: &str) -> Result<Vec<u8>, String> {
    dokki::compile(context.ctx, source)
}

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
