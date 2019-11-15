#![recursion_limit = "256"]

//TODO: переместить в этот файл RootView
//TODO: выделить папку для компонентов, и для вьюх
#[path = "./root_view.rs"]
pub mod root;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::start_app::<root::RootView>();
    Ok(())
}

#[wasm_bindgen]
pub fn get_number() -> u8 {
    3
}
