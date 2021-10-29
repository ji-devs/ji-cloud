use serde::{de, Deserializer,Deserialize};
use serde_repr::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::fmt;
use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Shape {
    pub path: Vec<PathPoint>,

    #[serde(rename="filePathRecording1")]
    pub audio: String,

    #[serde(rename="filePathRecording2")]
    pub audio_2: String,

    #[serde(rename="filePathThumb")]
    pub image_thumb: String,

    pub settings: ShapeSettings,

    #[serde(rename="pk")]
    pub key: PrimaryKey
}

#[derive(Deserialize, Debug, Clone)]
pub struct PathPoint {
    #[serde(rename="type")]
    pub kind: PathElementKind,

    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,

    #[serde(default)]
    pub cp1x: f64,
    #[serde(default)]
    pub cp1y: f64,
    #[serde(default)]
    pub cp2x: f64,
    #[serde(default)]
    pub cp2y: f64,
}
#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
pub enum PathElementKind {
    MoveToPoint,
    AddLineToPoint,
    AddQuadCurveToPoint,
    AddCurveToPoint,
    CloseSubPath
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShapeSettings {
    #[serde(rename="linkToPage")]
    pub jump_index: Option<usize>,

    #[serde(rename="toolTipText")]
    pub text: Option<String>,

    #[serde(rename="originTransform")]
    pub transform: Option<Transform>,

    #[serde(rename="filePathRecording1")]
    pub audio: Option<String>,

    #[serde(rename="textAnswerArray")]
    pub text_answers: Option<Vec<String>>,

    #[serde(rename="isUsingSpeakingMode")]
    pub speaking_mode: Option<bool>,

    #[serde(rename="highlightColor")]
    pub highlight_color: Option<String>,
}
