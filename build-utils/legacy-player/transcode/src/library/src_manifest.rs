use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;

pub mod activity;
use activity::*;

pub mod shape;
use shape::*;

pub mod layer;
use layer::*;


pub mod slide;
use slide::*;


#[derive(Deserialize, Debug)]
pub struct SrcManifest {
    /// Base url of the amazon bucket
    pub base_url: String,

    pub structure: ManifestStructure
}

impl SrcManifest {
    pub fn load(json:PathBuf) -> Self {
        let file = File::open(json).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct ManifestStructure {
    #[serde(rename="musicFile")]
    pub music_file: String,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    pub settings: ManifestSettings,

    #[serde(rename="shuffleType")]
    pub shuffle_type: ShuffleType,

    pub version: usize,

    pub slides: Vec<Slide>,
}

#[derive(Deserialize, Debug)]
pub struct ManifestSettings {
    #[serde(rename="DisableEditing")]
    pub disable_editing: u8,

    #[serde(rename="quizParameters")]
    pub quiz: QuizSettings,


}

#[derive(Deserialize, Debug)]
pub struct QuizSettings {
    #[serde(rename="activityTimeLimit")]
    pub activity_time_limit: u32,

    #[serde(rename="globalLivesLimit")]
    pub global_lives_limit: u32,

    #[serde(rename="globalTimeLimit")]
    pub global_time_limit: u32,

    #[serde(rename="quizModeEnabled")]
    pub enabled: bool,
}

pub type PrimaryKey = usize;

#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug)]
pub enum ShuffleType {
    None = 0,
    AllSlides = 1,
    Middle = 2 // All except first and last
}

//see: https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/matrix
pub type Transform = [f64;6];
