use std::collections::HashMap;

use once_cell::sync::OnceCell;

use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

use self::mixpanel::MixpanelProvider as Provider;

pub mod mixpanel;

static ANALYTICS: OnceCell<Provider> = OnceCell::new();

pub(crate) trait AnalyticsProvider {
    fn init() -> bool;
    fn event(&self, message: &str, properties: Option<HashMap<&str, String>>);
}

pub(crate) fn init() {
    if let None = ANALYTICS.get() {
        if Provider::init() {
            let _ = ANALYTICS.set(Provider);
        }
    }
}

pub fn event(message: &str, properties: Option<HashMap<&str, String>>) {
    // If for some reason Analytics hasn't been initialized, then we do nothing here.
    if let Some(provider) = ANALYTICS.get() {
        provider.event(message, properties);
    }
}

/// Implementation of serde_wasm_bindgen's `to_value` which serializes maps as objects instead of as a JS Map.
pub fn to_value<T: serde::ser::Serialize + ?Sized>(
    value: &T,
) -> Result<JsValue, serde_wasm_bindgen::Error> {
    value.serialize(&Serializer::new().serialize_maps_as_objects(true))
}
