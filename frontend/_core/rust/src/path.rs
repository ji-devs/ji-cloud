use super::settings::SETTINGS;
use config::MEDIA_UI_PATH;

pub fn ui<T: AsRef<str>>(path:T) -> String {
    media_url(&format!("{}/{}", MEDIA_UI_PATH, path.as_ref()))
}

pub fn media_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.media_url(), path)
}

pub fn api_url(path:&str) -> String {
    format!("{}{}", SETTINGS.get().unwrap().remote_target.api_url(), path)
}

pub fn upload_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.upload_url(), path)
}

pub fn upload_image_url(path:&str) -> String {
    format!("{}/image/{}", SETTINGS.get().unwrap().remote_target.upload_url(), path)
}

