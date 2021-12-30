use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;
use super::*;

#[derive(Deserialize, Debug)]
pub struct Slide {
    #[serde(rename="filePath")]
    pub file_path: String,

    #[serde(rename="filePathImage")]
    pub image_full: String,

    #[serde(rename="filePathImageThumb")]
    pub image_thumb: Option<String>,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    #[serde(rename="engineType", deserialize_with = "activity_kind_deser")]
    pub activity_kind: ActivityKind,

    pub layers: Vec<Layer>,

    pub activities: Vec<Activity>,
}

impl Slide {
    pub fn slide_id(&self) -> String {
        self.file_path
            .trim_matches('/')
            .replace("/", "-")
            .to_string()
    }
}