use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ModeExt, StepExt, ThemeId, _groups::design::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Poster(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Poster
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
            Body::Poster(data) => Ok(data),
            _ => Err("cannot convert body to poster!"),
        }
    }
}

/// The body for [`Poster`](crate::domain::jig::module::ModuleKind::Poster) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// The base content for all design modules
    pub base: BaseContent,
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
    /// Storytime
    StoryTime,
    /// Teach a word
    TeachAWord,
    /// TalkingPictures
    TalkingPictures,
    /// Poster
    Poster,
    /// Map
    Map,
    /// Hear a song
    HearASong,
    /// Printables
    Printables,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Poster
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::StoryTime,
            Self::TeachAWord,
            Self::TalkingPictures,
            Self::Poster,
            Self::Map,
            Self::HearASong,
            Self::Printables,
        ]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::StoryTime => "story-time",
            Self::TeachAWord => "teach-a-word",
            Self::TalkingPictures => "talking-pictures",
            Self::Poster => "poster",
            Self::Map => "map",
            Self::HearASong => "hear-a-song",
            Self::Printables => "printables",
        }
    }

    fn label(&self) -> &'static str {
        const STR_STORY_TIME_LABEL: &'static str = "Story time";
        const STR_TEACH_A_WORD_LABEL: &'static str = "Teach a word";
        const STR_TALKING_PICTURES_LABEL: &'static str = "Talking picture";
        const STR_POSTER_LABEL: &'static str = "Poster";
        const STR_MAP_LABEL: &'static str = "Map";
        const STR_HEAR_A_SONG_LABEL: &'static str = "Hear a song";
        const STR_PRINTABLES_LABEL: &'static str = "Printables";

        match self {
            Self::StoryTime => STR_STORY_TIME_LABEL,
            Self::TeachAWord => STR_TEACH_A_WORD_LABEL,
            Self::TalkingPictures => STR_TALKING_PICTURES_LABEL,
            Self::Poster => STR_POSTER_LABEL,
            Self::Map => STR_MAP_LABEL,
            Self::HearASong => STR_HEAR_A_SONG_LABEL,
            Self::Printables => STR_PRINTABLES_LABEL,
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
            Self::Four => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    fn label(&self) -> &'static str {
        const STR_DESIGN: &'static str = "Design";
        const STR_CONTENT: &'static str = "Content";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_DESIGN,
            Self::Two => STR_CONTENT,
            Self::Three => STR_SETTINGS,
            Self::Four => STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three, Self::Four]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}
