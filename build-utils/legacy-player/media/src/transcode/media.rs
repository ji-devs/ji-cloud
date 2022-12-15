use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use shared::domain::module::body::legacy::design::Animation;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaInfo {
    pub game_id: String,
    pub url: String,
    pub basepath: String,
    pub filename: String,
    pub transcode: Option<(MediaTranscode, String)>,
}

impl MediaInfo {
    pub fn file_stem(&self) -> String {
        std::path::Path::new(&self.filename).file_stem().unwrap().to_str().unwrap().to_string()
    }

    pub fn file_ext(&self) -> String {
        std::path::Path::new(&self.filename).extension().unwrap().to_str().unwrap().to_string()
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub enum MediaTranscode {
    Audio,
    Video
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscodeCommand {
    pub src: String, 
    pub dest: String,
    pub cmd: MediaTranscode
}