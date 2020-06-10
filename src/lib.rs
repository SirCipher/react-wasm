use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn say_hello() {
    alert("Hello from Rust!");
}
