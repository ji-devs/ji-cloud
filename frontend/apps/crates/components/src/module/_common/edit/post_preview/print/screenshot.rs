use js_sys::Reflect;
use shared::domain::{asset::AssetId, module::ModuleId};
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::JsValue;

pub fn print(asset_id: AssetId, module_id: ModuleId) {
    let html = format!(
        r#"
            <style>
                body {{
                    place-content: center;
                    display: grid;
                    height: 100vh;
                    width: 100vw;
                    margin: 0;
                }}
                img {{
                    max-width: 100vw;
                    max-height: 100vh;
                }}
            </style>
            <img src="https://uploads.sandbox.jicloud.org/screenshot/{}/{}/full.jpg">
        "#,
        asset_id.uuid(),
        module_id.0
    );

    let custom_elements_script = web_sys::window()
        .unwrap_ji()
        .document()
        .unwrap_ji()
        .query_selector("script[src$='/custom-elements.js']")
        .unwrap_ji()
        .unwrap_ji();

    let custom_elements_src = Reflect::get(&custom_elements_script, &JsValue::from_str("src"))
        .unwrap_ji()
        .as_string()
        .unwrap_ji();

    let scripts = vec![custom_elements_src];

    super::print(html, scripts);
}
