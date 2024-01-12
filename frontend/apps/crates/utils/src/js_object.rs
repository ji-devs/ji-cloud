pub use ::gloo;
pub use ::serde_json;

#[macro_export]
macro_rules! js_object {
    ($($value:tt)+) => {
        <wasm_bindgen::JsValue as $crate::js_object::gloo::utils::format::JsValueSerdeExt>::from_serde(&$crate::js_object::serde_json::json!($($value)+)).unwrap_ji()
    };
}
