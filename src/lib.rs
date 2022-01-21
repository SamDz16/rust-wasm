// mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_string(str: String) -> String {
    let r = String::from("hello ");
    return r + &str;
}

#[wasm_bindgen]
pub fn get_num(num: i32) -> i32 {
    return num;
}
