use ji_core::config::{
    ANIMATION_BODY_SIZE_LIMIT, AUDIO_BODY_SIZE_LIMIT, IMAGE_BODY_SIZE_LIMIT, PDF_BODY_SIZE_LIMIT,
};
use shared::media::{FileKind, PngImageFile};

pub struct Client;

impl Client {
    pub fn file_size_limit(file_kind: &FileKind) -> Option<usize> {
        match file_kind {
            FileKind::AnimationGif => Some(ANIMATION_BODY_SIZE_LIMIT),
            FileKind::ImagePng(PngImageFile::Original) => Some(IMAGE_BODY_SIZE_LIMIT),
            FileKind::AudioMp3 => Some(AUDIO_BODY_SIZE_LIMIT),
            FileKind::DocumentPdf => Some(PDF_BODY_SIZE_LIMIT),
            _ => unimplemented!("File type size limit undefined!"),
        }
    }
}
