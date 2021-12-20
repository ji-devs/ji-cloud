use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::{
    path::{Path, PathBuf},
    fs::File,
    fmt
};
use super::{
    slide::*,
    activity::*,
    layer::*,
    shape::*
};

#[derive(Deserialize, Debug)]
pub struct SrcManifest {
    /// Base url of the amazon bucket
    pub base_url: String,

    pub structure: ManifestStructure,

    pub album_store: AlbumStore
}


impl SrcManifest {
    pub fn game_id(&self) -> String {
        format!("{}", self.album_store.album.key)
    }

    pub fn lang_str(&self) -> &'static str {

        let tt_lang:u32 = self.album_store.album.fields.language.unwrap_or_default();

        match tt_lang { 
            16 => "da",
            8 => "nl",
            1 | 14 | 13 | 10 | 12 => "en",
            9 => "fr",
            11 => "de",
            2 => "he",
            18 => "hu",
            19 => "it",
            7 => "pt",
            6 => "ru",
            5 => "es",
            17 => "sv",
            _ => {
                log::warn!("no lang set, picking en");
                "en"
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ManifestStructure {
    #[serde(rename="musicFile")]
    pub music_file: Option<String>,

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
    #[serde(rename="quizParameters")]
    pub quiz: Option<QuizSettings>,
}

#[derive(Deserialize, Debug)]
pub struct QuizSettings {
    #[serde(rename="activityTimeLimit")]
    pub activity_time_limit: Option<f64>,

    #[serde(rename="globalLivesLimit")]
    pub global_lives_limit: Option<f64>,

    #[serde(rename="globalTimeLimit")]
    pub global_time_limit: Option<f64>,

    #[serde(rename="quizModeEnabled")]
    pub enabled: Option<bool>,
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

#[derive(Deserialize, Debug)]
pub struct AlbumStore {
    pub album: Album,

    #[serde(rename="pk")]
    pub key: PrimaryKey,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    #[serde(rename="pk")]
    pub key: PrimaryKey,
    pub fields: AlbumFields,
}

#[derive(Deserialize, Debug)]
pub struct AlbumFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<AlbumAuthor>,
    pub hash: Option<String>,
    pub language: Option<u32>,
}


#[derive(Deserialize, Debug)]
pub struct AlbumAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}