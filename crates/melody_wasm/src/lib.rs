extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsValue> {
    Ok(melody_compiler::compiler(&source).expect_throw("Encountered a parsing error"))
}
