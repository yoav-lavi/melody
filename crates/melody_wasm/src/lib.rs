#![allow(clippy::unused_unit)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsValue> {
    let output = melody_compiler::compiler(source);

    output.map_err(|error| {
        if error.detail.is_some() {
            JsValue::from(format!(
                "Error: {} on line {}",
                error.detail.unwrap(),
                error.line_index + 1
            ))
        } else {
            JsValue::from(format!(
                "Error: Unexpected \"{}\" on line {}",
                error.token,
                error.line_index + 1
            ))
        }
    })
}
