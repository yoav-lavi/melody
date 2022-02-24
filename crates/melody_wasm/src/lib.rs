#![allow(clippy::unused_unit)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsValue> {
    let raw_output = melody_compiler::compiler(source);
    if let Err(error) = raw_output {
        return Err(JsValue::from(format!(
            "Error: Unexpected \"{}\" on line {}",
            error.token,
            error.line_index + 1
        )));
    }
    Ok(raw_output.unwrap())
}
