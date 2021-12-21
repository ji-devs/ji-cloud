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
    pub intro_audio: Option<String>,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    #[serde(rename="folderPath")]
    pub folder_path: String,

    pub settings: Option<ActivitySettings>,

    pub shapes: Vec<Shape>
}

//Deserializing in place doesn't work since the parent object
//can be parsed in any order
//and so there's no way to depend on ActivityKind being read/set
//these are renamed to _try_ and make some comprehensive sense
//though tbh that's kinda futile (see github discussion: https://github.com/ji-devs/ji-cloud/discussions/1787)
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ActivitySettings {

    /// if set- advances automatically to next page
    pub advance: Option<bool>,

    /// the index to jump to
    #[serde(rename="linkToPage")]
    pub jump_index: Option<usize>,

    /// play one at a time is inverse of this?
    #[serde(rename="soundFunMode")]
    pub fun_mode: Option<bool>,

    /// play one at a time is inverse of this?
    #[serde(rename="soundFunModeV2")]
    pub fun_mode_v2: Option<bool>,

    #[serde(rename="soundHideHints")]
    pub hide_hints: Option<bool>,

    #[serde(rename="kIsShowSoundboardHintsOnStart")]
    pub hints_on_start: Option<bool>,

    #[serde(rename="kShowConfetti")]
    pub confetti: Option<bool>,

    #[serde(rename="BGRecording")]
    pub bg_audio: Option<String>,

    #[serde(rename="videoRange")]
    pub video_range: Option<VideoRange>,

    #[serde(rename="videoTitle")]
    pub video_title: Option<String>,
    
    #[serde(rename="videoURL")]
    pub video_url: Option<String>,

    #[serde(rename="videoThumbURL")]
    pub video_thumb_url: Option<String>, 

    pub transform: Option<Transform>,

    #[serde(rename="showShape")]
    pub show_shape: Option<bool>,

    #[serde(rename="showShapeV2")]
    pub show_shape_v2: Option<bool>,

    #[serde(rename="DisableHints")]
    pub hints_disabled: Option<bool>,

    #[serde(rename="ShapePuzzleTheme")]
    pub theme: Option<u8>,
    #[serde(rename="ShapePuzzleThemeV2")]
    pub theme_v2: Option<bool>,

    #[serde(rename="soundShowToolTip")]
    pub tooltip: Option<bool>,
}

