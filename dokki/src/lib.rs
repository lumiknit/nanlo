pub mod wasm;

/// Compile context for the nanlo language to WASM
pub struct CompileContext {}

/// Create a new compile context
pub fn new_context() -> CompileContext {
    CompileContext {}
}

/// Compile the given source code to WASM
/// Returns the compiled WASM, or an error message
pub fn compile(ctx: &mut CompileContext, filename: &str, src: &str) -> Result<Vec<u8>, String> {
    Ok(wasm::create_empty_module())
}
