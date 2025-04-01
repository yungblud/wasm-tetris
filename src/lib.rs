use wasm_bindgen::prelude::*;

// JS에서 호출 할 수 있도록 설정
#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from Rust!".to_string()
}