use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ModeExt, ThemeChoice, _groups::cards::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Flashcards`](crate::domain::jig::module::ModuleKind::Flashcards) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`Flashcards`](crate::domain::jig::module::ModuleKind::Flashcards) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Content {
    /// The base content for all cards modules
    pub base: BaseContent,
    /// Settings for playback
    pub player_settings: PlayerSettings,
}

/// Player settings
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct PlayerSettings {
    /// display mode
    pub display_mode: DisplayMode,
}

/// Display Mode
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum DisplayMode {
    /// Single sided cards
    Single,
    /// Double sided cards
    Double,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Single
    }
}

impl DisplayMode {
    /// Get it as a string
    pub fn as_str_id(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Double => "double",
        }
    }
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Flashcards(self.clone())
    }

    fn choose_mode_list() -> Vec<Mode> {
        Mode::get_list()
            .into_iter()
            .filter(|mode| *mode != Mode::Duplicate)
            .collect()
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Flashcards
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
            ModuleKind::Matching,
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
    fn convert_to_matching(&self) -> Result<super::matching::ModuleData, &'static str> {
        Ok(super::matching::ModuleData {
            content: self
                .content
                .as_ref()
                .map(|content| super::matching::Content {
                    base: content.base.clone(),
                    player_settings: super::matching::PlayerSettings::default(),
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
            Body::Flashcards(data) => Ok(data),
            _ => Err("cannot convert body to flashcards!"),
        }
    }
}
