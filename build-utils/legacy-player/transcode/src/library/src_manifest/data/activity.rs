use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;
use super::*;

 //example: "{0, 1262.1670999999999}"
    //not a lawful json string...
pub type VideoRange = String;

#[repr(u8)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ActivityKind {
    None,
    SaySomething,
    Soundboard,
    Video,
    Puzzle,
    Questions,
    TalkType,
}

pub fn activity_kind_deser<'de, D>(deserializer: D) -> Result<ActivityKind, D::Error>
where
    D: Deserializer<'de>,
{
    let s:String = Deserialize::deserialize(deserializer)?;

    match s.as_ref() {
        "S" => Ok(ActivityKind::None),
        "R" => Ok(ActivityKind::SaySomething),
        "A" => Ok(ActivityKind::Soundboard),
        "V" => Ok(ActivityKind::Video),
        "P" => Ok(ActivityKind::Puzzle),
        "Q" => Ok(ActivityKind::Questions),
        "T" => Ok(ActivityKind::TalkType),
        _ => Err(serde::de::Error::custom(format!("unknown activity type [{}]!", s)))
    }

}

#[derive(Deserialize, Debug, Clone)]
pub struct Activity {
    #[serde(rename="filePathIntroRecording")]
    pub intro_audio: String,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    #[serde(rename="folderPath")]
    pub folder_path: String,

    pub settings: ActivitySettings,

    pub shapes: Vec<Shape>
}

//Deserializing in place doesn't work since the parent object
//can be parsed in any order
//and so there's no way to depend on ActivityKind being read/set
#[derive(Deserialize, Debug, Clone)]
pub struct ActivitySettings {

    /// if set- advances automatically to next page
    pub advance: Option<bool>,

    /// the index to jump to
    #[serde(rename="linkToPage")]
    pub jump_index: Option<usize>,


    #[serde(rename="soundFunModeV2")]
    pub fun_mode: Option<bool>,

    #[serde(rename="soundHideHints")]
    pub hide_hints: Option<bool>,

    #[serde(rename="kIsShowSoundboardHintsOnStart")]
    pub hints_on_start: Option<bool>,

    #[serde(rename="kShowConfetti")]
    pub confetti: Option<bool>,

    #[serde(rename="BGRecording")]
    pub bg_audio: Option<String>,

    #[serde(rename="videoRange")]
    pub range: Option<VideoRange>,

    #[serde(rename="videoTitle")]
    pub title: Option<String>,
    
    #[serde(rename="videoURL")]
    pub url: Option<String>,

    pub transform: Option<Transform>,

    #[serde(rename="videoThumbURL")]
    pub img_thumb: Option<String>, 


    #[serde(rename="showShapeV2")]
    pub show_shape: Option<bool>,

    #[serde(rename="DisableHints")]
    pub hints_disabled: Option<bool>,

    #[serde(rename="ShapePuzzleThemeV2")]
    pub theme: Option<bool>,


    #[serde(rename="soundShowToolTip")]
    pub tooltip: Option<bool>,
}

