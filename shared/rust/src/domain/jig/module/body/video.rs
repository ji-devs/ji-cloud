use crate::domain::jig::module::{
    body::{Body, BodyExt, ModeExt, StepExt, ThemeChoice, _groups::design::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

use super::BodyConvert;

/// The body for [`Video`](crate::domain::jig::module::ModuleKind::Video) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Video(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Video
    }
    fn new_mode(mode: Mode) -> Self {
        ModuleData {
            content: Some(Content {
                mode,
                ..Content::default()
            }),
        }
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

    fn get_theme(&self) -> Option<ThemeChoice> {
        self.content.as_ref().map(|content| content.base.theme)
    }
}

impl BodyConvert for ModuleData {}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Video(data) => Ok(data),
            _ => Err("cannot convert body to video!"),
        }
    }
}

/// The body for [`Video`](crate::domain::jig::module::ModuleKind::Video) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The editor state
    pub editor_state: EditorState,

    /// The mode
    pub mode: Mode,

    /// The base content for all design modules
    pub base: BaseContent,

    /// Play settings
    pub play_settings: PlaySettings,
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
    /// Introduction
    Introduction,
    /// Story
    Story,
    /// Song
    Song,
    /// Howto
    Howto,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Introduction
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![Self::Introduction, Self::Story, Self::Song, Self::Howto]
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Introduction => "introduction",
            Self::Story => "story",
            Self::Song => "song",
            Self::Howto => "howto",
        }
    }

    fn label(&self) -> &'static str {
        const STR_INTRODUCTION_LABEL: &'static str = "Introduction";
        const STR_STORY_LABEL: &'static str = "Story";
        const STR_SONG_LABEL: &'static str = "Song";
        const STR_HOWTO_LABEL: &'static str = "How to";

        match self {
            Self::Introduction => STR_INTRODUCTION_LABEL,
            Self::Story => STR_STORY_LABEL,
            Self::Song => STR_SONG_LABEL,
            Self::Howto => STR_HOWTO_LABEL,
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
        //TODO - localizaton
        const STR_BACKGROUND: &'static str = "Design";
        const STR_CONTENT: &'static str = "Content";
        const STR_SETTINGS: &'static str = "Settings";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_BACKGROUND,
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

/// Video play settings
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaySettings {
    /// show captions
    pub captions: bool,

    /// play with sound
    pub muted: bool,

    /// autoplay
    pub autoplay: bool,

    /// what to do when done
    pub done_action: Option<DoneAction>,
}

/// what to do when done playing video
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum DoneAction {
    /// loop the video
    Loop,

    /// move on to next activity
    Next,
}
