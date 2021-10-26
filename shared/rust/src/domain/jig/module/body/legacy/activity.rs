pub use super::path::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Activity {
    Questions(Questions),
    SaySomething(SaySomething),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Questions {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub audio: String,
    pub path: Vec<PathPoint>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaySomething {
    pub audio_filename: String,
    pub advance_trigger: AdvanceTrigger,
    pub advance_index: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AdvanceTrigger {
    AudioEnd,
    Tap,
}
