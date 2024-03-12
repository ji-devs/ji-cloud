use crate::domain::module::{
    body::{Body, BodyExt, ModeExt, StepExt, ThemeId, _groups::design::*},
    ModuleKind,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::BodyConvert;

/// The body for [`Embed`](crate::domain::module::ModuleKind::Embed) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<Mode, Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Embed(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Embed
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
            Body::Embed(data) => Ok(data),
            _ => Err("cannot convert body to embed!"),
        }
    }
}

/// The body for [`Embed`](crate::domain::module::ModuleKind::Embed) modules.
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, EnumIter)]
/// The mode
pub enum Mode {
    /// Class quiz
    Quiz,
    /// Worksheets
    Worksheets,
    /// Project-based learning
    ProjectBasedLearning,
    /// Class project
    ClassProject,
    /// Portfolio
    Portfolio,
    /// Forms
    Forms,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Quiz
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        Self::iter().collect()
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Quiz => "quiz",
            Self::Worksheets => "worksheets",
            Self::ProjectBasedLearning => "project-based-learning",
            Self::ClassProject => "class-project",
            Self::Portfolio => "portfolio",
            Self::Forms => "forms",
        }
    }

    fn label(&self) -> &'static str {
        const STR_QUIZ: &str = "Class quiz";
        const STR_WORKSHEETS: &str = "Worksheets";
        const STR_PROJECT_BASED_LEARNING: &str = "Project-based learning";
        const STR_CLASS_PROJECT: &str = "Class project";
        const STR_DIGITAL_PORTFOLIO: &str = "Digital portfolio";
        const STR_COLLABORATIVE_FORMS: &str = "Collaborative forms";

        match self {
            Mode::Quiz => STR_QUIZ,
            Mode::Worksheets => STR_WORKSHEETS,
            Mode::ProjectBasedLearning => STR_PROJECT_BASED_LEARNING,
            Mode::ClassProject => STR_CLASS_PROJECT,
            Mode::Portfolio => STR_DIGITAL_PORTFOLIO,
            Mode::Forms => STR_COLLABORATIVE_FORMS,
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
