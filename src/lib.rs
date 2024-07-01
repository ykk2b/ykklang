use wasm_bindgen::prelude::*;
mod api;
mod backend;
mod frontend;
mod tests;

#[wasm_bindgen]
extern "C" {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn interpret(source_code: &str) {
    let interpreter = backend::interpreter::Interpreter::new();
    let statements = frontend::frontend(source_code);
    backend::backend(statements, interpreter);
}