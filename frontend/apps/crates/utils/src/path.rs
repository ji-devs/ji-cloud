use super::settings::SETTINGS;
use config::MEDIA_UI_PATH;
use shared::{
    media::{media_key,MediaLibrary, FileKind, PngImageFile},
    domain::image::ImageId
};
use wasm_bindgen::prelude::*;

pub fn ui<T: AsRef<str>>(path:T) -> String {
    media_url(&format!("{}/{}", MEDIA_UI_PATH, path.as_ref()))
}

pub fn library_image_id(library_kind: MediaLibrary, img_kind: PngImageFile, id:ImageId) -> String {
    let path = media_key(library_kind, id.0, FileKind::ImagePng(img_kind));

    uploads_url(&path)
}
pub fn library_image_str(library_kind: MediaLibrary, img_kind: PngImageFile, id:&str) -> String {
    let id = uuid::Uuid::parse_str(id).unwrap_throw();
    let path = media_key(library_kind, id, FileKind::ImagePng(img_kind));
    uploads_url(&path)
}

pub fn uploads_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.uploads_url(), path)
}
pub fn media_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.media_url(), path)
}
