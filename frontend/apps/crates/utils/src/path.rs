use super::init::settings::SETTINGS;
use shared::{
    domain::audio::AudioId,
    domain::{image::ImageId, pdf::PdfId},
    media::{media_key, FileKind, MediaLibrary, PngImageFile},
};

use crate::unwrap::UnwrapJiExt;

pub fn ui<T: AsRef<str>>(path: T) -> String {
    media_url(&format!("ui/{}", path.as_ref()))
}

pub fn legacy_cdn_url<T: AsRef<str>>(path: T) -> String {
    if crate::routes::is_param_bool("example") {
        media_url(&format!("legacy/examples/{}", path.as_ref()))
    } else {
        legacy_url(&format!("games/{}", path.as_ref()))
    }
}

pub fn config_cdn_url<T: AsRef<str>>(path: T) -> String {
    media_url(&format!("config/{}", path.as_ref()))
}

pub fn audio_cdn_url<T: AsRef<str>>(path: T) -> String {
    media_url(&format!("audio/{}", path.as_ref()))
}

pub fn audio_lib_url(library_kind: MediaLibrary, id: AudioId) -> String {
    let path = media_key(library_kind, id.0, FileKind::AudioMp3);

    uploads_url(&path)
}

pub fn image_lib_url(library_kind: MediaLibrary, img_kind: PngImageFile, id: ImageId) -> String {
    let path = media_key(library_kind, id.0, FileKind::ImagePng(img_kind));

    uploads_url(&path)
}

pub fn pdf_lib_url(library_kind: MediaLibrary, id: PdfId) -> String {
    let path = media_key(library_kind, id.0, FileKind::DocumentPdf);

    uploads_url(&path)
}

pub fn uploads_url(path: &str) -> String {
    format!(
        "{}/{}",
        SETTINGS.get().unwrap_ji().remote_target.uploads_url(),
        path
    )
}
pub fn media_url(path: &str) -> String {
    format!(
        "{}/{}",
        SETTINGS.get().unwrap_ji().remote_target.media_url(),
        path
    )
}

pub fn legacy_url(path: &str) -> String {
    format!(
        "{}/{}",
        SETTINGS.get().unwrap_ji().remote_target.legacy_url(),
        path
    )
}
