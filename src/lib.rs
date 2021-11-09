mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {} ", name));
}

#[wasm_bindgen]
pub fn double (num: i32) -> i32 {
    return num+num;
}
#[wasm_bindgen]
pub fn triple (num: i32) -> i32 {
    return num+num+num;
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
#[wasm_bindgen]
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => factorial(n - 1) * n,
    }
}
