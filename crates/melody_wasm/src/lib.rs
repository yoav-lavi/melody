#![forbid(unsafe_code)]
#![allow(clippy::unused_unit)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsValue> {
    let output = melody_compiler::compiler(source);
    output.map_err(|error| JsValue::from(format!("Error: {}", error.message)))
}
