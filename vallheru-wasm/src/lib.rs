mod util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_name() -> String {
    util::name_generator::random_name()
}
