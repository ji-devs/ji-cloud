use super::settings::SETTINGS;
use config::MEDIA_UI_PATH;
use shared::{
    media::{image_id_to_key,MediaLibraryKind, MediaVariant},
    domain::image::ImageId
};
use wasm_bindgen::prelude::*;

pub fn ui<T: AsRef<str>>(path:T) -> String {
    media_url(&format!("{}/{}", MEDIA_UI_PATH, path.as_ref()))
}

pub fn library_image(library_kind: MediaLibraryKind, variant: MediaVariant, id:&str) -> String {
    let id = uuid::Uuid::parse_str(id).unwrap_throw();
    let path = image_id_to_key(library_kind, variant, ImageId(id));

    uploads_url(&path)
}

fn uploads_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.uploads_url(), path)
}
fn media_url(path:&str) -> String {
    format!("{}/{}", SETTINGS.get().unwrap().remote_target.media_url(), path)
}
