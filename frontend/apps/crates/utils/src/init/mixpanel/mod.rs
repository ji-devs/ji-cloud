use std::collections::HashMap;

use once_cell::sync::OnceCell;

use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

use crate::env::env_var;

/// This is necessary so that we
/// - Don't reinitialize Mixpanel, and
/// - Don't call any of its functions before Mixpanel has been initialized.
///
/// Mixpanel's JS library uses a singleton that can be globally initialized and used.
static MIXPANEL: OnceCell<()> = OnceCell::new();

// Hackity hack. See comment in /js/mixpanel.js
#[wasm_bindgen(module = "/js/mixpanel.js")]
extern "C" {
    type Mixpanel;

    #[wasm_bindgen(js_name = "init", static_method_of = Mixpanel)]
    fn init(token: String, config: &JsValue);

    #[wasm_bindgen(js_name = "track", static_method_of = Mixpanel)]
    fn track(message: &str, properties: &JsValue);
}

/// Rust version of Mixpanel's Config type.
///
/// Add properties when they're needed.
#[derive(Serialize)]
struct MixpanelConfig {
    pub debug: bool,
}

pub(crate) fn init() {
    if let None = MIXPANEL.get() {
        // No token, no tracking
        if let Ok(token) = env_var("MIXPANEL_PROJECT_TOKEN") {
            let debug = cfg!(feature = "local"); // Enable debug mode only for local development
            let config = MixpanelConfig { debug };

            // Only initialize Mixpanel if everything is
            if let Ok(config) = to_value(&config) {
                Mixpanel::init(token.into(), &config);
                let _ = MIXPANEL.set(());
            }
        }
    }
}

pub fn track(message: &str, properties: Option<HashMap<&str, String>>) {
    // If for some reason Mixpanel hasn't been initialized, then we do nothing here.
    if let Some(_) = MIXPANEL.get() {
        // Tracking shouldn't break the UI if there's ever an issue.
        if let Ok(value) = to_value(&properties) {
            Mixpanel::track(message, &value);
        }
    }
}

/// Implementation of serde_wasm_bindgen's `to_value` which serializes maps as objects instead of as a JS Map.
///
/// Mixpanel's library doesn't support using the Map type.
pub fn to_value<T: serde::ser::Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&Serializer::new().serialize_maps_as_objects(true))
}
