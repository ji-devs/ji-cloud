use components::module::_common::edit::prelude::*;
use components::audio::mixer::AudioMixer;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            StepExt,
            ThemeChoice,
            Audio,
            Instructions,
            tapping_board::{
                Step,
                PlaySettings as RawPlaySettings, 
                Hint, Next,
                Mode, 
                Content as RawContent, 
                ModuleData as RawData
            },
            _groups::design::{
                Trace as RawTrace,
                Backgrounds as RawBackgrounds, 
                TraceKind,
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
        state::{Stickers,Sticker},
        callbacks::Callbacks as StickersCallbacks
    },
    backgrounds::{
        state::Backgrounds,
        callbacks::Callbacks as BackgroundsCallbacks,
    },
    traces::{
        bubble::TraceBubble,
        edit::{TracesEdit, TracesEditCallbacks},
    },
    tooltip::state::State as TooltipState
};
use dominator::clone;
use std::cell::RefCell;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    // TappingBoard-specific
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers<Sticker>>, 
    pub traces: Rc<TracesEdit>,
    pub traces_meta: MutableVec<TraceMeta>,
    pub text_editor: Rc<TextEditorState>,
    pub audio_mixer: AudioMixer,
    pub play_settings: Rc<PlaySettings>,
}

pub struct PlaySettings {
    pub hint: Mutable<Hint>,
    pub next: Mutable<Next>,
    pub next_value: Mutable<usize>
}

impl PlaySettings {
    pub fn new(settings:RawPlaySettings) -> Self {

        let next_value = Mutable::new(
                match &settings.next {
                    Next::SelectSome(value) => *value,
                    _ => {
                        crate::config::DEFAULT_SELECT_AMOUNT
                    }
                },
            );
        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
            next_value
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
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {

        let BaseInitFromRawArgs { 
            raw,
            jig_id,
            module_id,
            history,
            step,
            theme_choice,
            theme_id,
            audio_mixer,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.base.instructions);
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers<Sticker>>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(
            theme_id.clone(),
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
                &content.base.backgrounds,
                theme_id.clone(),
                BackgroundsCallbacks::new(
                    Some(clone!(history => move |raw_bgs| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.base.backgrounds = raw_bgs;
                            }
                        });
                    }))
                )
        ));


        let stickers = Stickers::new(
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |stickers:&[Sticker]| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.base.stickers = stickers 
                                    .iter()
                                    .map(|sticker| {
                                        sticker.to_raw()
                                    })
                                    .collect();
                            }
                        });
                    }))
                )
        );
       
        stickers.replace_all(
            content.base.stickers.clone()
                .iter()
                .map(|raw_sticker| {
                    Sticker::new(stickers.clone(), raw_sticker)
                })
                .collect::<Vec<Sticker>>()
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());


        let traces = TracesEdit::from_raw(

            &content.traces
                .iter()
                .map(|trace_meta| {
                    trace_meta.trace.clone()
                })
                .collect::<Vec<RawTrace>>(),
            crate::debug::settings().draw_kind.unwrap_or(TraceKind::Regular),
            TracesEditCallbacks::new(
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
            theme_id,
            history,
            step: step.read_only(),
            theme_choice,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            traces,
            traces_meta,
            audio_mixer,
            play_settings: Rc::new(PlaySettings::new(content.play_settings.clone())),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> { 
        self.theme_id.signal().map(|id| id.as_str_id())
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

    fn get_jig_id(&self) -> JigId {
        self.jig_id
    }
    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }

}
