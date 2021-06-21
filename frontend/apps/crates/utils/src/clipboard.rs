use wasm_bindgen::prelude::*;

// using inline_js because the web_sys Clipboard is unstable
#[wasm_bindgen(inline_js = "
    export function _writeText(value) {
        navigator.clipboard.writeText(value);
    }
")]
extern "C" {
    fn _writeText(value: &str);
}

pub fn write_text(value: &str) {
    _writeText(value);
}
