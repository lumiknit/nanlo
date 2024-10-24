use wasm_encoder::Module;

/// Build empty module
pub fn create_empty_module() -> Vec<u8> {
    let module = Module::new();
    module.finish()
}
