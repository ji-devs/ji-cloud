use std::path::PathBuf;

use shared::domain::jig::module::body::legacy::design::Animation;
use uuid::Uuid;
pub struct Media {
    pub url: String,
    pub basepath: String,
    pub filename: String,
    pub transcode: Option<MediaTranscode>,
}

impl Media {
    pub fn file_stem(&self) -> String {
        std::path::Path::new(&self.filename).file_stem().unwrap().to_str().unwrap().to_string()
    }

    pub fn file_ext(&self) -> String {
        std::path::Path::new(&self.filename).extension().unwrap().to_str().unwrap().to_string()
    }
}

pub enum MediaTranscode {
    Audio
}