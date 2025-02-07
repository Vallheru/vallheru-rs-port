mod util;

extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_name() -> String {
    util::name_generator::random_name()
}
