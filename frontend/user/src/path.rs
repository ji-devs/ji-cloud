use crate::settings::SETTINGS;

pub fn ui<T: AsRef<str>>(path:T) -> String {
    media_url(&format!("app/ui/{}", path.as_ref()))
}

pub fn media_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.media_url_base, path)
}

pub fn api_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.api_url_base, path)
}

pub fn api_js_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.api_js_url_base, path)
}