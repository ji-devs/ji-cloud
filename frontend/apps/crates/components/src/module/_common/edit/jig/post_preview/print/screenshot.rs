use js_sys::Reflect;
use shared::domain::{asset::AssetId, module::ModuleId};
use utils::{path::uploads_url, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;

pub fn print(asset_id: AssetId, module_id: ModuleId) {
    let screenshot_url = format!(
        "screenshot/{}/{}/full.jpg?cb={}",
        asset_id.uuid(),
        module_id.0,
        js_sys::Math::random().to_string()
    );
    let screenshot_url = uploads_url(&screenshot_url);

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
            div {{
                display: grid;
            }}
            img {{
                max-width: 100vw;
                max-height: 100vh;
                grid-column: 1;
                grid-row: 1;
            }}
            span {{
                grid-column: 1;
                grid-row: 1;
                align-self: end;
                color: #fff;
                display: flex;
                align-items: center;
                padding: 1vh 1vw;
            }}
            span img-ui {{
                height: 16px;
            }}
        </style>
        <div>
            <img src="{screenshot_url}">
            <span>
                <img-ui path="core/page-header/logo.svg"></img-ui>.org
            </span>
        </div>
    "#
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
