use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::poster::{Mode as RawMode, Content as RawContent, ModuleData as RawData}}};
use futures_signals::signal::{ReadOnlyMutable, Mutable};

pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>
}

impl Base {
    pub fn new(history: Rc<HistoryStateImpl<RawData>>, step: ReadOnlyMutable<Step>) -> Self {
        Self {
            history,
            step
        }
    }
}

impl BaseExt<Step> for Base {
    fn allowed_step_change(&self, from:Step, to:Step) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two, 
    Three,
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
        match self {
            Self::One => crate::strings::steps_nav::STR_DESIGN,
            Self::Two => crate::strings::steps_nav::STR_CONTENT,
            Self::Three => crate::strings::steps_nav::STR_SETTINGS,
            Self::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four
        ]
    }
    fn get_preview() -> Self {
        Self::Four
    }
}
