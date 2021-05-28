use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    module::{
        ModuleId, 
        body::{
            Backgrounds as RawBackgrounds, 
            tapping_board::{Mode as RawMode, Content as RawContent, ModuleData as RawData}
        }
    }
};
use futures_signals::signal::{ReadOnlyMutable, Mutable};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
    stickers::state::Stickers,
    backgrounds::state::Backgrounds,
    traces::edit::state::Edit as TracesEdit
};
use dominator::clone;
use std::cell::RefCell;

pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,

    // TappingBoard-specific
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers>, 
    pub traces: Rc<TracesEdit>, 
    pub text_editor: Rc<TextEditorState>,
}

impl Base {
    pub fn new(is_history: bool, history: Rc<HistoryStateImpl<RawData>>, step: ReadOnlyMutable<Step>, raw: Option<&RawContent>) -> Self {

        let theme_id = match raw {
            None => ThemeId::None,
            Some(raw) => raw.theme_id
        };
       
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(theme_id, None, 
            Some(Box::new(clone!(stickers_ref => move |value| {
                if let Some(stickers) = stickers_ref.borrow().as_ref() {
                    stickers.set_current_text_value(value.to_string());
                }
            }))),
            Some(Box::new(clone!(stickers_ref => move || {
                if let Some(stickers) = stickers_ref.borrow().as_ref() {
                    stickers.current_text_blur();
                }
            })))
        );

        let backgrounds = Rc::new(Backgrounds::new(
                raw.map(|content| &content.backgrounds),
                None
        ));

        let stickers = Stickers::new(
                raw.map(|content| content.stickers.as_ref()),
                text_editor.clone(),
                Some(clone!(history => move |raw_stickers| {
                    history.push_modify(|raw| {
                        if let Some(content) = &mut raw.content {
                            content.stickers = raw_stickers;
                        }
                    });
                }))
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let traces = TracesEdit::new(
                raw.map(|content| content.traces.as_ref()),
                crate::debug::settings().trace_opts.clone(),
                Some(clone!(history => move |raw_traces| {
                    history.push_modify(|raw| {
                        if let Some(content) = &mut raw.content {
                            content.traces = raw_traces;
                        }
                    });
                }))
        );

        Self {
            history,
            step,
            theme_id: Mutable::new(theme_id),
            text_editor,
            backgrounds,
            stickers,
            traces
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
    Five,
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
            Self::Four => Some(Self::Five),
            Self::Five => None,
        }
    }

    fn as_number(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 4,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::One => crate::strings::steps_nav::STR_BACKGROUND,
            Self::Two => crate::strings::steps_nav::STR_CONTENT,
            Self::Three => crate::strings::steps_nav::STR_INTERACTION,
            Self::Four => crate::strings::steps_nav::STR_SETTINGS,
            Self::Five => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    fn get_list() -> Vec<Self> {
        vec![
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five
        ]
    }
    fn get_preview() -> Self {
        Self::Five
    }
}
