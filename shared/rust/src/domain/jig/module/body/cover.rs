use crate::domain::jig::module::{
    body::{Body, BodyExt, StepExt, ThemeChoice, _groups::design::*},
    ModuleKind,
};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::convert::TryFrom;

/// The body for [`Cover`](crate::domain::jig::module::ModuleKind::Cover) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ModuleData {
    /// The content
    pub content: Option<Content>,
}

impl BodyExt<(), Step> for ModuleData {
    fn as_body(&self) -> Body {
        Body::Cover(self.clone())
    }

    fn is_complete(&self) -> bool {
        self.content.is_some()
    }

    fn kind() -> ModuleKind {
        ModuleKind::Cover
    }
    fn new_mode(_mode: ()) -> Self {
        ModuleData {
            content: Some(Content::default()),
        }
    }

    fn requires_choose_mode(&self) -> bool {
        false
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

impl TryFrom<Body> for ModuleData {
    type Error = &'static str;

    fn try_from(body: Body) -> Result<Self, Self::Error> {
        match body {
            Body::Cover(data) => Ok(data),
            _ => Err("cannot convert body to cover!"),
        }
    }
}

/// The body for [`Cover`](crate::domain::jig::module::ModuleKind::Cover) modules.
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct Content {
    /// The editor state
    pub editor_state: EditorState,

    /// The base content for all design modules
    pub base: BaseContent,
}

/// Editor state
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct EditorState {
    /// the current step
    pub step: Step,

    /// the completed steps
    pub steps_completed: HashSet<Step>,
}

/// The Steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
        const STR_THEMES: &'static str = "Themes";
        const STR_BACKGROUND: &'static str = "Background";
        const STR_CONTENT: &'static str = "Content";
        const STR_PREVIEW: &'static str = "Preview";

        match self {
            Self::One => STR_THEMES,
            Self::Two => STR_BACKGROUND,
            Self::Three => STR_CONTENT,
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
