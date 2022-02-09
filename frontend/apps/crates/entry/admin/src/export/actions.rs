use chrono::{DateTime, Utc, TimeZone};
use futures_signals::signal::Mutable;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::JsValue;

use super::Export;

impl Export {
    pub fn set_date(&self, date: &Mutable<Option<DateTime<Utc>>>, value: &JsValue) {
        let js_date = js_sys::Date::new(value);
        let datetime = chrono::Utc.datetime_from_str(
            &js_date.to_iso_string().as_string().unwrap_ji(),
            "%Y-%m-%dT%H:%M:%S%Z"
        );
        if let Ok(datetime) = datetime {
            date.set(Some(datetime));
        }
    }
}
