use std::collections::HashMap;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::env::env_var;

use super::{to_value, AnalyticsProvider};

// Hackity hack. See comment in /js/mixpanel.js
#[wasm_bindgen(module = "/js/mixpanel.js")]
extern "C" {
    pub type Mixpanel;

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

pub(crate) struct MixpanelProvider;

impl AnalyticsProvider for MixpanelProvider {
    fn init() -> bool {
        if let Ok(token) = env_var("MIXPANEL_PROJECT_TOKEN") {
            let debug = cfg!(feature = "local"); // Enable debug mode only for local development
            let config = MixpanelConfig { debug };

            // Only initialize Mixpanel if everything is
            if let Ok(config) = to_value(&config) {
                Mixpanel::init(token.into(), &config);
            }

            return true;
        }

        false
    }

    fn event(&self, message: &str, properties: Option<HashMap<&str, String>>) {
        // Tracking shouldn't break the UI if there's ever an issue.
        if let Ok(value) = to_value(&properties) {
            Mixpanel::track(message, &value);
        }
    }
}
