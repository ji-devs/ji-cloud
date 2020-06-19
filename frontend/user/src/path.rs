use crate::settings::SETTINGS;
use ji_cloud_shared::frontend::settings::MEDIA_UI_PATH;

pub fn ui<T: AsRef<str>>(path:T) -> String {
    media_url(&format!("{}/{}", MEDIA_UI_PATH, path.as_ref()))
}

pub fn media_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.media_url_base(), path)
}

pub fn api_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.api_url_base(), path)
}

pub fn api_js_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.api_js_url_base(), path)
}
