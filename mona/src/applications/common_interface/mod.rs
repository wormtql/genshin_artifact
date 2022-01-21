use wasm_bindgen::prelude::*;

pub mod get_attribute;

pub struct CommonInterface {}

#[wasm_bindgen]
impl CommonInterface {
    fn get_attribute(val: &JsValue) -> JsValue {
        get_attribute::get_attribute(val)
    }
}