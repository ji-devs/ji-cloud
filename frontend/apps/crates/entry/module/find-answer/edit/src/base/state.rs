use components::module::_common::edit::prelude::*;

use components::module::_groups::design::edit::design_ext::DesignExt;
use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{Sticker, Stickers},
    },
    text_editor::{TextEditor, TextEditorCallbacks},
    traces::edit::{TracesEdit, TracesEditCallbacks},
};
use dominator::clone;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable},
    signal_vec::MutableVec,
};
use shared::domain::asset::AssetId;
use shared::domain::module::body::_groups::design::Trace;
use shared::domain::module::body::find_answer::{Ordering, Question as RawQuestion};
use shared::domain::{
    jig::JigId,
    module::{
        body::{
            BodyExt, Instructions,
            _groups::design::TraceKind,
            find_answer::{
                Mode, ModuleData as RawData, Next, PlaySettings as RawPlaySettings, Step,
            },
        },
        ModuleId,
    },
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    // FindAnswer-specific
    pub phase: Mutable<Phase>,
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub questions: MutableVec<Rc<Question>>,
    pub current_question: Mutable<Option<usize>>,
    pub text_editor: Rc<TextEditor>,
    pub play_settings: Rc<PlaySettings>,
    pub continue_next_fn: ContinueNextFn,
}

pub struct PlaySettings {
    pub ordering: Mutable<Ordering>,
    pub has_attempts_limit: Mutable<bool>,
    pub n_attempts: Mutable<u8>,
    pub has_time_limit: Mutable<bool>,
    pub time_limit: Mutable<u32>,
    pub next: Mutable<Next>,
    pub next_value: Mutable<usize>,
}

const DEFAULT_ATTEMPTS_LIMIT: u8 = 2;
const DEFAULT_TIME_LIMIT: u32 = 3;

impl PlaySettings {
    pub fn new(settings: RawPlaySettings) -> Self {
        let next_value = Mutable::new(match &settings.next {
            Next::SelectSome(value) => *value,
            _ => crate::config::DEFAULT_SELECT_AMOUNT,
        });
        Self {
            ordering: Mutable::new(settings.ordering),
            has_attempts_limit: Mutable::new(settings.n_attempts.is_some()),
            n_attempts: Mutable::new(settings.n_attempts.unwrap_or(DEFAULT_ATTEMPTS_LIMIT)),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
            time_limit: Mutable::new(settings.time_limit.unwrap_or(DEFAULT_TIME_LIMIT)),
            next: Mutable::new(settings.next),
            next_value,
        }
    }
}

/// Represents a single question
#[derive(Clone)]
pub struct Question {
    /// Title of the question
    pub title: Mutable<Option<String>>,

    /// The question text
    pub question_text: Mutable<Option<String>>,

    /// Optional audio for the question
    pub question_audio: Mutable<Option<Audio>>,

    /// Optional text for incorrect choices
    pub incorrect_text: Mutable<Option<String>>,

    /// Optional audio for incorrect choices
    pub incorrect_audio: Mutable<Option<Audio>>,

    /// Traces
    pub traces: Rc<MutableVec<Trace>>,

    pub is_editing_title: Mutable<bool>,

    pub confirm_delete: Mutable<bool>,
}

impl Question {
    fn from_raw_vec(questions: Vec<RawQuestion>) -> MutableVec<Rc<Question>> {
        MutableVec::new_with_values(
            questions
                .iter()
                .map(|question| Rc::new(Question::from(question)))
                .collect(),
        )
    }
}

impl Default for Question {
    fn default() -> Self {
        Self {
            title: Mutable::new(None),
            question_text: Mutable::new(None),
            question_audio: Mutable::new(None),
            incorrect_text: Mutable::new(None),
            incorrect_audio: Mutable::new(None),
            traces: Rc::new(MutableVec::new()),
            is_editing_title: Mutable::new(false),
            confirm_delete: Mutable::new(false),
        }
    }
}

impl From<&RawQuestion> for Question {
    fn from(raw_question: &RawQuestion) -> Self {
        Self {
            title: Mutable::new(Some(raw_question.title.clone())),
            question_text: Mutable::new(Some(raw_question.question_text.clone())),
            question_audio: Mutable::new(raw_question.question_audio.clone()),
            incorrect_text: Mutable::new(raw_question.incorrect_text.clone()),
            incorrect_audio: Mutable::new(raw_question.incorrect_audio.clone()),
            traces: Rc::new(MutableVec::new_with_values(raw_question.traces.clone())),
            is_editing_title: Mutable::new(false),
            confirm_delete: Mutable::new(false),
        }
    }
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            asset_id,
            module_id,
            history,
            step,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.base.instructions);

        let stickers_ref: Rc<RefCell<Option<Rc<Stickers<Sticker>>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditor::new(
            theme_id.read_only(),
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
                })),
            ),
        );

        let backgrounds = Rc::new(Backgrounds::from_raw(
            &content.base.backgrounds,
            theme_id.read_only(),
            BackgroundsCallbacks::new(Some(clone!(history => move |raw_bgs| {
                history.push_modify(|raw| {
                    if let Some(content) = &mut raw.content {
                        content.base.backgrounds = raw_bgs;
                    }
                });
            }))),
        ));

        let stickers = Stickers::new(
            text_editor.clone(),
            StickersCallbacks::new(Some(clone!(history => move |stickers:&[Sticker]| {
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
            }))),
        );

        stickers.replace_all(
            content
                .base
                .stickers
                .iter()
                .map(|raw_sticker| Sticker::new(stickers.clone(), raw_sticker))
                .collect::<Vec<Sticker>>(),
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let _self = Rc::new(Self {
            jig_id: asset_id.unwrap_jig().clone(),
            module_id,
            theme_id,
            history,
            step: step.read_only(),
            instructions,
            text_editor,
            phase: Mutable::new(Phase::Layout),
            backgrounds,
            stickers,
            questions: Question::from_raw_vec(content.questions),
            current_question: Mutable::new(None),
            play_settings: Rc::new(PlaySettings::new(content.play_settings)),
            continue_next_fn: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }
}

impl BaseExt<Step> for Base {
    fn allowed_step_change(&self, _from: Step, _to: Step) -> bool {
        true
    }

    fn can_continue_next(&self) -> ReadOnlyMutable<bool> {
        Mutable::new(true).read_only()
    }

    fn continue_next(&self) -> bool {
        match self.step.get() {
            Step::Two | Step::Three | Step::Four => match self.continue_next_fn.get_cloned() {
                Some(continue_next_fn) => continue_next_fn(),
                None => false,
            },
            _ => false,
        }
    }

    fn get_asset_id(&self) -> AssetId {
        AssetId::JigId(self.jig_id)
    }

    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }
}

impl DesignExt for Base {
    fn get_backgrounds(&self) -> Rc<Backgrounds> {
        Rc::clone(&self.backgrounds)
    }

    fn get_theme(&self) -> Mutable<ThemeId> {
        self.theme_id.clone()
    }

    fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set(theme);

        self.history.push_modify(|raw| {
            raw.set_theme(theme);
        });
    }
}

#[derive(Clone)]
pub enum Phase {
    /// Layout phase
    Layout,
    /// Trace phase with trace state.
    ///
    /// Note: We could upgrade futures_signals so that we have the Clone implementation for
    /// MutableVec<T>, however, there is a note in the code to remove Arc from the MutableVec
    /// struct and I don't won't to commit to that clone if it could to break things in the
    /// future.
    ///
    /// Also, see comment: https://github.com/Pauan/rust-signals/pull/50#issuecomment-1028840756
    Trace(Rc<TracesEdit>),
}

impl Phase {
    pub fn new_trace_unchecked(state: Rc<Base>, trace_kind: TraceKind) -> Self {
        let content = &state.history.get_current().content.unwrap_ji();

        let raw_traces = &content
            .questions
            .get(state.current_question.get().unwrap_ji())
            .map_or_else(|| vec![], |question| question.traces.clone());

        Self::Trace(TracesEdit::from_raw(
            raw_traces,
            crate::debug::settings().draw_kind.unwrap_or(trace_kind),
            TracesEditCallbacks::new(
                Some(clone!(state => move |raw_trace| {
                    state.on_trace_added(raw_trace);
                })),
                Some(clone!(state => move |index| {
                    state.on_trace_deleted(index);
                })),
                Some(clone!(state => move |index, raw_trace| {
                    state.on_trace_changed(index, raw_trace);
                })),
            ),
        ))
    }
}