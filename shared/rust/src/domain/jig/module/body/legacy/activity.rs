use crate::domain::jig::module::body::_groups::design::TraceShape;

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaySomething {
    pub audio_filename: String,
    pub advance_trigger: AdvanceTrigger,
    pub advance_index: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Soundboard {
    pub items: Vec<SoundboardItem>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundboardItem {
    //pub audio: String,
    pub shape: TraceShape,
}

// used in multiple activities
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdvanceTrigger {
    AudioEnd,
    Tap,
}