use crate::domain::jig::module::{
    body::{Body, BodyExt, ThemeChoice, _groups::cards::*},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`CardQuiz`](crate::domain::jig::module::ModuleKind::CardQuiz) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`CardQuiz`](crate::domain::jig::module::ModuleKind::CardQuiz) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The base content for all cards modules
    pub base: BaseContent,
    /// Settings for playback
    pub player_settings: PlayerSettings,
}

/// Player settings
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct PlayerSettings {
    /// number of choices
    pub n_choices: u8,

    /// swap the display to be primary left vs. right
    pub swap: bool,

    /// number of rounds to play
    pub n_rounds: u32,

    /// time limit in minutes
    pub time_limit: Option<u32>,

    /// number of attempts
    pub n_attempts: Option<u8>,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            n_choices: 3,
            swap: false,
            n_rounds: 1,
            time_limit: None,
            n_attempts: None,
        }
    }
}
/// Module is a memory game, and has a memory game's body.
impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::CardQuiz(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::CardQuiz
    }

    fn new_mode(mode: Mode) -> Self {
        ModuleData {
            content: Some(Content {
                base: BaseContent::new(mode),
                ..Content::default()
            }),
        }
    }

    fn requires_choose_mode(&self) -> bool {
        self.content.is_none()
    }

    fn set_editor_state_step(&mut self, step: Step) {
        if let Some(content) = self.content.as_mut() {
            content.base.editor_state.step = step;
        }
    }
    fn set_editor_state_steps_completed(&mut self, steps_completed: HashSet<Step>) {
        if let Some(content) = self.content.as_mut() {
            content.base.editor_state.steps_completed = steps_completed;
        }
    }

    fn get_editor_state_step(&self) -> Option<Step> {
        self.content
            .as_ref()
            .map(|content| content.base.editor_state.step)
    }

    fn get_editor_state_steps_completed(&self) -> Option<HashSet<Step>> {
        self.content
            .as_ref()
            .map(|content| content.base.editor_state.steps_completed.clone())
    }

    fn get_theme(&self) -> Option<ThemeChoice> {
        self.content.as_ref().map(|content| content.base.theme)
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::CardQuiz(data) => Ok(data),
            _ => Err("cannot convert body to flashcards!"),
        }
    }
}
