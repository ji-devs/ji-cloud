use components::module::edit::*;
use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    module::{
        ModuleId, 
        body::{
            Trace as RawTrace,
            Backgrounds as RawBackgrounds, 
            Audio,
            tapping_board::{Mode as RawMode, Content as RawContent, ModuleData as RawData}
        }
    }
};
use futures_signals::{
    signal::{ReadOnlyMutable, Mutable},
    signal_vec::MutableVec
};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
    stickers::state::Stickers,
    backgrounds::state::Backgrounds,
    traces::{
        bubble::state::TraceBubble,
        edit::{
            state::Edit as TracesEdit, 
            callbacks::Callbacks as TracesCallbacks
        }
    },
    tooltip::state::State as TooltipState
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
    pub traces_meta: MutableVec<TraceMeta>,
    pub text_editor: Rc<TextEditorState>,
    pub audio_ctx: AudioContext
}

#[derive(Clone)]
pub struct TraceMeta {
    pub audio: Mutable<Option<Audio>>,
    pub text: Mutable<Option<String>>,
    pub bubble: Mutable<Option<Rc<TraceBubble>>>,
}

impl TraceMeta {
    pub fn new(audio: Option<Audio>, text: Option<String>) -> Self {
        Self {
            audio: Mutable::new(audio),
            text: Mutable::new(text),
            bubble: Mutable::new(None)
        }
    }
}

impl Base {
    pub fn new(is_history: bool, history: Rc<HistoryStateImpl<RawData>>, step: ReadOnlyMutable<Step>, raw: Option<&RawContent>) -> Rc<Self> {

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let theme_id = match raw {
            None => ThemeId::None,
            Some(raw) => raw.theme_id
        };
       
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(theme_id, None, 
            Some(clone!(stickers_ref => move |value:&str| {
                if let Some(stickers) = stickers_ref.borrow().as_ref() {
                    Stickers::add_text(stickers.clone(), value.to_string());
                }
            })),
            Some(clone!(stickers_ref => move |value:&str| {
                if let Some(stickers) = stickers_ref.borrow().as_ref() {
                    stickers.set_current_text_value(value.to_string());
                }
            })),
            Some(clone!(stickers_ref => move || {
                if let Some(stickers) = stickers_ref.borrow().as_ref() {
                    stickers.current_text_blur();
                }
            }))
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

        let raw_traces:Option<Vec<RawTrace>> = 
            raw.map(|content| {
                    content.traces
                        .iter()
                        .map(|trace_meta| {
                            trace_meta.trace.clone()
                        })
                        .collect()
            });


        let traces = TracesEdit::new(
                raw_traces.as_ref().map(|x| x.as_slice()),
                crate::debug::settings().trace_opts.clone(),
                TracesCallbacks::new(
                    Some(clone!(_self_ref => move |raw_trace| {
                        if let Some(_self) = _self_ref.borrow().as_ref() {
                            _self.on_trace_added(raw_trace);
                        }
                    })),
                    Some(clone!(_self_ref => move |index| {
                        if let Some(_self) = _self_ref.borrow().as_ref() {
                            _self.on_trace_deleted(index);
                        }
                    })),
                    Some(clone!(_self_ref => move |index, raw_trace| {
                        if let Some(_self) = _self_ref.borrow().as_ref() {
                            _self.on_trace_changed(index, raw_trace);
                        }
                    })),
                )
        );

        let traces_meta = MutableVec::new_with_values(
            raw.map(|content| {
                    content.traces
                        .iter()
                        .map(|trace_meta| {
                            TraceMeta::new(
                                trace_meta.audio.clone(), 
                                trace_meta.text.clone()
                            )
                        })
                        .collect()
            }).unwrap_or_default()
        );

        let _self = Rc::new(Self {
            history,
            step,
            theme_id: Mutable::new(theme_id),
            text_editor,
            backgrounds,
            stickers,
            traces,
            traces_meta,
            audio_ctx: AudioContext::new().unwrap_ji()
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
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
