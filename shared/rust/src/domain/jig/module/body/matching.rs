use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ThemeChoice, _groups::cards::*},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Matching`](crate::domain::jig::module::ModuleKind::Matching) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`Matching`](crate::domain::jig::module::ModuleKind::Matching) modules.
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
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            n_choices: 3,
            swap: false,
            n_rounds: 1,
            time_limit: None,
        }
    }
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Matching(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Matching
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

impl BodyConvert for ModuleData {
    fn convertable_list() -> Vec<ModuleKind> {
        vec![
            ModuleKind::Memory,
            ModuleKind::Flashcards,
            ModuleKind::CardQuiz,
        ]
    }
    fn convert_to_memory(&self) -> Result<super::memory::ModuleData, &'static str> {
        Ok(super::memory::ModuleData {
            content: self.content.as_ref().map(|content| super::memory::Content {
                base: content.base.clone(),
                player_settings: super::memory::PlayerSettings::default(),
            }),
        })
    }
    fn convert_to_flashcards(&self) -> Result<super::flashcards::ModuleData, &'static str> {
        Ok(super::flashcards::ModuleData {
            content: self
                .content
                .as_ref()
                .map(|content| super::flashcards::Content {
                    base: content.base.clone(),
                    player_settings: super::flashcards::PlayerSettings::default(),
                }),
        })
    }

    fn convert_to_card_quiz(&self) -> Result<super::card_quiz::ModuleData, &'static str> {
        Ok(super::card_quiz::ModuleData {
            content: self
                .content
                .as_ref()
                .map(|content| super::card_quiz::Content {
                    base: content.base.clone(),
                    player_settings: super::card_quiz::PlayerSettings::default(),
                }),
        })
    }
}

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Matching(data) => Ok(data),
            _ => Err("cannot convert body to flashcards!"),
        }
    }
}
