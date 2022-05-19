use crate::domain::module::{
    body::{
        Body, BodyConvert, BodyExt, ModeExt, StepExt, ThemeId,
        _groups::design::{BaseContent, Trace},
    },
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

mod play_settings;
pub use play_settings::*;

use super::{Audio, Transform};

/// The body for [`FindAnswer`](crate::domain::module::ModuleKind::FindAnswer) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::FindAnswer(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::FindAnswer
    }

    fn new_with_mode_and_theme(mode: Mode, theme: ThemeId) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                base: BaseContent {
                    theme,
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
    }

    fn mode(&self) -> Option<Mode> {
        self.content.as_ref().map(|c| c.mode.clone())
    }

    fn requires_choose_mode(&self) -> bool {
        self.content.is_none()
    }

    fn set_editor_state_step(&mut self, step: Step) {
        if let Some(content) = self.content.as_mut() {
            content.editor_state.step = step;
        }
    }
    fn set_editor_state_steps_completed(&mut self, steps_completed: HashSet<Step>) {
        if let Some(content) = self.content.as_mut() {
            content.editor_state.steps_completed = steps_completed;
        }
    }

    fn get_editor_state_step(&self) -> Option<Step> {
        self.content
            .as_ref()
            .map(|content| content.editor_state.step)
    }

    fn get_editor_state_steps_completed(&self) -> Option<HashSet<Step>> {
        self.content
            .as_ref()
            .map(|content| content.editor_state.steps_completed.clone())
    }

    fn set_theme(&mut self, theme_id: ThemeId) {
        if let Some(content) = self.content.as_mut() {
            content.base.theme = theme_id;
        }
    }

    fn get_theme(&self) -> Option<ThemeId> {
        self.content.as_ref().map(|content| content.base.theme)
    }
}

impl BodyConvert for ModuleData {}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::FindAnswer(data) => Ok(data),
            _ => Err("cannot convert body to Find the Answer!"),
        }
    }
}

/// The body for [`FindAnswer`](crate::domain::module::ModuleKind::FindAnswer) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The base content for all design modules
    pub base: BaseContent,

    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// Questions
    pub questions: Vec<Question>,

    /// Sticker index of the related question sticker
    pub question_field: QuestionField,

    /// play settings
    pub play_settings: PlaySettings,
}

/// The type of field to be used for displaying question text.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum QuestionField {
    /// Index of the text sticker to be used as the question field.
    Text(usize),
    /// When the teacher hasn't added or selected a text sticker, a dynamic label will be added to
    /// display the question. The teacher can move this around.
    ///
    /// Note (Ty): We won't make use of the scale field right now, but at some point we should add
    /// the ability to scale the label text
    Dynamic(Transform),
}

impl Default for QuestionField {
    fn default() -> Self {
        QuestionField::Dynamic(Transform::default())
    }
}

/// Represents a single question
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Question {
    /// Title of the question
    pub title: String,

    /// The question text
    pub question_text: String,

    /// Optional audio for the question
    pub question_audio: Option<Audio>,

    /// Optional text for incorrect choices
    pub incorrect_text: Option<String>,

    /// Optional audio for incorrect choices
    pub incorrect_audio: Option<Audio>,

    /// Traces
    pub traces: Vec<Trace>,
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
/// The mode
pub enum Mode {
    /// Family mode
    Family,
    /// Map mode
    Map,
    /// Multiple mode
    MultipleChoice,
    /// Scene mode
    Scene,
    /// Text mode
    Text,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Family
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Family,
            Self::Map,
            Self::MultipleChoice,
            Self::Scene,
            Self::Text,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Family => "family",
            Self::Map => "map",
            Self::MultipleChoice => "multiple-choice",
            Self::Scene => "scene",
            Self::Text => "text",
        }
    }

    fn label(&self) -> &'static str {
        const STR_FAMILY_LABEL: &'static str = "Family";
        const STR_MAP_LABEL: &'static str = "Map";
        const STR_MULTIPLE_CHOICE_LABEL: &'static str = "Multiple Choice";
        const STR_SCENE_LABEL: &'static str = "Scene";
        const STR_TEXT_LABEL: &'static str = "Text";

        match self {
            Self::Family => STR_FAMILY_LABEL,
            Self::Map => STR_MAP_LABEL,
            Self::MultipleChoice => STR_MULTIPLE_CHOICE_LABEL,
            Self::Scene => STR_SCENE_LABEL,
            Self::Text => STR_TEXT_LABEL,
        }
    }
}

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    /// Step 1
    One,
    /// Step 2
    Two,
    /// Step 3
    Three,
    /// Step 4
    Four,
    /// Step 5
    Five,
}

impl Default for Step {
    fn default() -> Self {
        Self::One
    }
}

impl StepExt for Step {
    fn next(&self) -> Option<Self> {
        match self {
            Self::One => Some(Self::Two),
            Self::Two => Some(Self::Three),
            Self::Three => Some(Self::Four),
            Self::Four => Some(Self::Five),
            Self::Five => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
        }
    }

    fn label(&self) -> &'static str {
        const STR_BACKGROUND: &'static str = "Design";
        const STR_CONTENT: &'static str = "Content";
        const STR_INTERACTION: &'static str = "Interaction";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";
        match self {
            Self::One => STR_BACKGROUND,
            Self::Two => STR_CONTENT,
            Self::Three => STR_INTERACTION,
            Self::Four => STR_SETTINGS,
            Self::Five => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four, Self::Five]
    }
    fn get_preview() -> Self {
        Self::Five
    }
}
