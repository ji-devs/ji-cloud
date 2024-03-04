// TODO:
// find some easy way to cache/static object, since we can't do `const fn` here.
// from js_sys::Date and chrono, and js-temporal.

use super::utils::options;

pub struct DateTimeFormat {
    js: js_sys::Intl::DateTimeFormat,
}

impl DateTimeFormat {
    pub fn new(locale: &str, options: Options) -> Self {
        let options = options.to_js_object();
        let locals = js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(locale));
        let js = js_sys::Intl::DateTimeFormat::new(&locals, &options);
        Self { js }
    }
    pub fn format(&self, date: &js_sys::Date) -> String {
        self.js
            .format()
            .call1(&self.js, &date.into())
            .unwrap()
            .as_string()
            .unwrap()
    }
}

#[derive(Clone, Default)]
pub struct Options {
    pub year: Option<Year>,
    pub month: Option<Month>,
    pub day: Option<Day>,
    pub hour: Option<Hour>,
    pub minute: Option<Minute>,
    pub second: Option<Second>,
}
impl Options {
    // TODO: should be a derive macro?
    fn to_js_object(&self) -> js_sys::Object {
        let output = js_sys::Object::new();
        if let Some(year) = &self.year {
            pro(&output, "year", year.string_rep());
        }
        if let Some(month) = &self.month {
            pro(&output, "month", month.string_rep());
        }
        if let Some(day) = &self.day {
            pro(&output, "day", day.string_rep());
        }
        if let Some(hour) = &self.hour {
            pro(&output, "hour", hour.string_rep());
        }
        if let Some(minute) = &self.minute {
            pro(&output, "minute", minute.string_rep());
        }
        if let Some(second) = &self.second {
            pro(&output, "second", second.string_rep());
        }
        output
    }
}

fn pro(obj: &js_sys::Object, key: &str, value: &str) {
    js_sys::Reflect::set(
        &obj,
        &wasm_bindgen::JsValue::from_str(key),
        &wasm_bindgen::JsValue::from_str(value),
    )
    .unwrap();
}

options!(Year, {
    Numeric: "numeric",
    Digit2: "2-digit",
});
options!(Month, {
    Numeric: "numeric",
    Digit2: "2-digit",
    Long: "long",
    Short: "short",
    Narrow: "narrow",
});
options!(Day, {
    Numeric: "numeric",
    Digit2: "2-digit",
});
options!(Hour, {
    Numeric: "numeric",
    Digit2: "2-digit",
});
options!(Minute, {
    Numeric: "numeric",
    Digit2: "2-digit",
});
options!(Second, {
    Numeric: "numeric",
    Digit2: "2-digit",
});
