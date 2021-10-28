use crate::domain::jig::module::body::_groups::design::TraceShape;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    Questions(Questions),
    SaySomething(SaySomething),
    Soundboard(Soundboard),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Questions {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub audio: String,
    pub shape: TraceShape,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaySomething {
    pub advance_trigger: AdvanceTrigger,

    pub audio_filename: Option<String>,

    pub advance_index: Option<usize>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Soundboard {
    pub audio_filename: Option<String>,
    pub items: Vec<SoundboardItem>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundboardItem {
    pub audio_filename: Option<String>,
    pub text: Option<String>,
    pub jump_index: Option<usize>,
    pub hotspot: Hotspot,
}

// used in multiple activities
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdvanceTrigger {
    AudioEnd,
    Tap,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hotspot {
    pub shape: TraceShape,
    pub transform_matrix: Option<[f64; 16]>,
}
