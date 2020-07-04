extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::env;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn system_report() {
    let path = env::var("PATH").is_err();
    alert(&format!("path {}", path));
}