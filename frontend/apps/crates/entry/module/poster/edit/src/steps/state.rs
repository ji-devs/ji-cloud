use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    module::{
        ModuleId, 
        body::{
            Backgrounds as RawBackgrounds, 
            poster::{Mode as RawMode, Content as RawContent, ModuleData as RawData}
        }
    }
};
use futures_signals::signal::{ReadOnlyMutable, Mutable};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
    stickers::state::Stickers,
    backgrounds::state::Backgrounds,
};

pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,

    // Poster-specific
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers>, 
    pub text_editor: Rc<TextEditorState>,
}

impl Base {
    pub fn new(history: Rc<HistoryStateImpl<RawData>>, step: ReadOnlyMutable<Step>, raw: Option<&RawContent>) -> Self {

        let theme_id = match raw {
            None => ThemeId::None,
            Some(raw) => raw.theme_id
        };
        
        let text_editor = TextEditorState::new(theme_id, None, None);

        let backgrounds = Rc::new(Backgrounds::new(
                raw.map(|content| &content.backgrounds),
                None
        ));

        let stickers = Rc::new(Stickers::new(
                raw.map(|content| content.stickers.as_ref()),
                text_editor.clone(),
                None
        ));

        Self {
            history,
            step,
            theme_id: Mutable::new(theme_id),
            text_editor,
            backgrounds,
            stickers
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
            Self::One => crate::strings::steps_nav::STR_THEMES,
            Self::Two => crate::strings::steps_nav::STR_BACKGROUND,
            Self::Three => crate::strings::steps_nav::STR_CONTENT,
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
