use components::module::edit::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Trace as RawTrace,
            Backgrounds as RawBackgrounds, 
            Audio,
            Instructions,
            tapping_board::{
                PlaySettings as RawPlaySettings, 
                Hint, Next,
                Mode as RawMode, 
                Content as RawContent, 
                ModuleData as RawData
            }
        }
    }
};
use futures_signals::{
    map_ref,
    signal::{self, Signal, SignalExt, ReadOnlyMutable, Mutable},
    signal_vec::MutableVec
};
use utils::prelude::*;
use components::{
    text_editor::{
        state::State as TextEditorState,
        callbacks::Callbacks as TextEditorCallbacks
    },
    stickers::{
        state::Stickers,
        callbacks::Callbacks as StickersCallbacks
    },
    backgrounds::{
        state::Backgrounds,
        callbacks::Callbacks as BackgroundsCallbacks,
    },
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
    pub theme: Mutable<ThemeChoice>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: Option<Jig>,
    // TappingBoard-specific
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers>, 
    pub traces: Rc<TracesEdit>,
    pub traces_meta: MutableVec<TraceMeta>,
    pub text_editor: Rc<TextEditorState>,
    pub audio_ctx: AudioContext,
    pub play_settings: Rc<PlaySettings>,
}

pub struct PlaySettings {
    pub hint: Mutable<Hint>,
    pub next: Mutable<Next>
}

impl PlaySettings {
    pub fn new(settings:RawPlaySettings) -> Self {

        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
        }
    }

    pub fn to_raw(&self) -> RawPlaySettings {
        RawPlaySettings {
            hint: self.hint.get_cloned(),
            next: self.next.get_cloned(),
        }
    }
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

    pub async fn new(
        jig_id: JigId,
        module_id: ModuleId,
        jig: Option<Jig>,
        raw:RawData, 
        step: ReadOnlyMutable<Step>,
        history: Rc<HistoryStateImpl<RawData>>
    ) -> Rc<Self> {

        let content = raw.content.unwrap_ji();

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let theme = Mutable::new(content.theme);
        let instructions = Mutable::new(content.instructions);
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(

            match content.theme {
                ThemeChoice::Jig => {
                    // self.jig.as_ref().unwrap_ji().theme_id.clone()
                    log::warn!("waiting on jig settings");
                    ThemeId::Chalkboard
                },
                ThemeChoice::Override(theme_id) => theme_id
            },
            None, 
            TextEditorCallbacks::new(
                //New text
                Some(clone!(stickers_ref => move |value:&str| {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        Stickers::add_text(stickers.clone(), value.to_string());
                    }
                })),
                //Text change
                Some(clone!(stickers_ref => move |value:&str| {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        stickers.set_current_text_value(value.to_string());
                    }
                })),
                //Blur
                Some(clone!(stickers_ref => move || {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        stickers.stop_current_text_editing();
                    }
                }))
        ));


        let backgrounds = Rc::new(Backgrounds::from_raw(
                &content.backgrounds,
                BackgroundsCallbacks::new(
                    Some(clone!(history => move |raw_bgs| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.backgrounds = raw_bgs;
                            }
                        });
                    }))
                )
        ));

        let stickers = Stickers::from_raw(
                &content.stickers,
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |raw_stickers| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.stickers = raw_stickers;
                            }
                        });
                    }))
                )
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());


        let traces = TracesEdit::from_raw(

            &content.traces
                .iter()
                .map(|trace_meta| {
                    trace_meta.trace.clone()
                })
                .collect::<Vec<RawTrace>>(),
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
            content.traces
                .iter()
                .map(|trace_meta| {
                    TraceMeta::new(
                        trace_meta.audio.clone(), 
                        trace_meta.text.clone()
                    )
                })
                .collect()
        );

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            jig,
            history,
            step,
            theme,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            traces,
            traces_meta,
            audio_ctx: AudioContext::new().unwrap_ji(),
            play_settings: Rc::new(PlaySettings::new(content.play_settings.clone())),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn get_theme_id(&self) -> ThemeId {
        match self.theme.get_cloned() {
            ThemeChoice::Jig => {
                // self.jig.as_ref().unwrap_ji().theme_id.clone()
                log::warn!("waiting on jig settings");
                ThemeId::Chalkboard
            },
            ThemeChoice::Override(theme_id) => theme_id
        }
    }
    pub fn theme_id_signal(&self) -> impl Signal<Item = ThemeId> {
        self.theme.signal_cloned()
            .map(|theme| match theme {
                ThemeChoice::Jig => {
                    // self.jig.as_ref().unwrap_ji().theme_id.clone()
                    log::warn!("waiting on jig settings");
                    ThemeId::Chalkboard
                },
                ThemeChoice::Override(theme_id) => theme_id
            })
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id_signal().map(|id| id.as_str_id())
    }

}


impl BaseExt<Step> for Base {
    type NextStepAllowedSignal = impl Signal<Item = bool>;

    fn allowed_step_change(&self, from:Step, to:Step) -> bool {
        true
    }

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal {
        signal::always(true)
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
            Self::Five => 5,
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
