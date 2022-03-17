use crate::domain::jig::module::{
    body::{Body, BodyConvert, BodyExt, ThemeId, _groups::cards::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

/// The content for [`Memory`](crate::domain::jig::module::ModuleKind::Memory) modules.
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
    /// time limit in minutes
    pub time_limit: Option<u32>,
    /// amount of pairs to render
    pub pairs_to_display: Option<u32>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::MemoryGame(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content
            .as_ref()
            .map_or(false, |content| content.base.is_valid())
    }

    fn kind() -> ModuleKind {
        ModuleKind::Memory
    }

    fn new_with_mode_and_theme(mode: Mode, theme: ThemeId) -> Self {
        ModuleData {
            content: Some(Content {
                base: BaseContent {
                    mode,
                    theme,
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
    }

    fn mode(&self) -> Option<Mode> {
        self.content.as_ref().map(|c| c.base.mode.clone())
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

    fn set_theme(&mut self, theme_id: ThemeId) {
        if let Some(content) = self.content.as_mut() {
            content.base.theme = theme_id;
        }
    }

    fn get_theme(&self) -> Option<ThemeId> {
        self.content.as_ref().map(|content| content.base.theme)
    }
}

impl BodyConvert for ModuleData {
    fn convertable_list() -> Vec<ModuleKind> {
        vec![
            ModuleKind::Matching,
            ModuleKind::Flashcards,
            ModuleKind::CardQuiz,
        ]
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
            Body::MemoryGame(data) => Ok(data),
            _ => Err("cannot convert body to memory game!"),
        }
    }
}
