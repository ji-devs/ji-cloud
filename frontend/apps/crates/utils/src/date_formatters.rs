use chrono::{DateTime, Utc};

use crate::intl;

thread_local! {
    pub(super) static YEAR_MONTH_DAY_HOUR_MINUTE_FORMATTER: intl::date_time_format::DateTimeFormat =
        intl::date_time_format::DateTimeFormat::new(
            "en-us",
            intl::date_time_format::Options {
                year: Some(intl::date_time_format::Year::Numeric),
                month: Some(intl::date_time_format::Month::Short),
                day: Some(intl::date_time_format::Day::Numeric),
                hour: Some(intl::date_time_format::Hour::Numeric),
                minute: Some(intl::date_time_format::Minute::Numeric),
                ..Default::default()
            },
        );
    pub(super) static YEAR_MONTH_DAY_FORMATTER: intl::date_time_format::DateTimeFormat =
        intl::date_time_format::DateTimeFormat::new(
            "en-us",
            intl::date_time_format::Options {
                year: Some(intl::date_time_format::Year::Numeric),
                month: Some(intl::date_time_format::Month::Short),
                day: Some(intl::date_time_format::Day::Numeric),
                ..Default::default()
            },
        );
    pub(super) static HOUR_MINUTE_FORMATTER: intl::date_time_format::DateTimeFormat =
        intl::date_time_format::DateTimeFormat::new(
            "en-us",
            intl::date_time_format::Options {
                hour: Some(intl::date_time_format::Hour::Numeric),
                minute: Some(intl::date_time_format::Minute::Numeric),
                ..Default::default()
            },
        );
}

pub fn year_month_day_hour_minute(date: &DateTime<Utc>) -> String {
    let js_millis = wasm_bindgen::JsValue::from_f64(date.timestamp_millis() as f64);
    let js_date = js_sys::Date::new(&js_millis);
    YEAR_MONTH_DAY_HOUR_MINUTE_FORMATTER.with(|formatter| formatter.format(&js_date))
}

pub fn year_month_day(date: &DateTime<Utc>) -> String {
    let js_millis = wasm_bindgen::JsValue::from_f64(date.timestamp_millis() as f64);
    let js_date = js_sys::Date::new(&js_millis);
    YEAR_MONTH_DAY_FORMATTER.with(|formatter| formatter.format(&js_date))
}

pub fn hour_minute(date: &DateTime<Utc>) -> String {
    let js_millis = wasm_bindgen::JsValue::from_f64(date.timestamp_millis() as f64);
    let js_date = js_sys::Date::new(&js_millis);
    HOUR_MINUTE_FORMATTER.with(|formatter| formatter.format(&js_date))
}
