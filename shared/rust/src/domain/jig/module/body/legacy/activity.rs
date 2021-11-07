use crate::domain::jig::module::body::_groups::design::{TraceShape, YoutubeUrl};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    AskQuestions(AskQuestions),
    SaySomething(SaySomething),
    Soundboard(Soundboard),
    Video(Video),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AskQuestions {
    pub items: Vec<QuestionItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionItem {
    pub question_filename: Option<String>,
    pub answer_filename: Option<String>,
    pub wrong_filename: Option<String>,
    pub hotspot: Hotspot,
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
    pub bg_audio_filename: Option<String>,
    /// this isn't actually used for anything
    pub highlight_color: Option<String>,
    /// this isn't actually used for anything
    pub one_at_a_time: bool,
    pub show_hints: bool,
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub transform_matrix: Option<[f64; 16]>,
    pub src: VideoSource,
    pub range: Option<(f64, f64)>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VideoSource {
    Youtube(YoutubeUrl),
    Direct(String),
}

////////// used in multiple activities
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
