use components::image::tag::ImageTag;
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
use futures_signals::signal_vec::VecDiff;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable},
    signal_vec::MutableVec,
};
use shared::domain::asset::AssetId;
use shared::domain::module::body::Audio;
use shared::domain::module::body::_groups::design::Trace;
use shared::domain::module::body::find_answer::{
    Ordering, Question as RawQuestion, QuestionField, DEFAULT_ATTEMPTS_LIMIT,
};
use shared::domain::{
    jig::JigId,
    module::{
        body::{
            BodyExt, ModuleAssist,
            _groups::design::TraceKind,
            find_answer::{Mode, ModuleData as RawData, PlaySettings as RawPlaySettings, Step},
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
    pub instructions: Mutable<ModuleAssist>,
    pub feedback: Mutable<ModuleAssist>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    // FindAnswer-specific
    pub phase: Mutable<Phase>,
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub questions: MutableVec<Rc<Question>>,
    pub question_field: Mutable<QuestionField>,
    // Original text that was in the question sticker. Required so that we can update
    // the sticker with its original text when all questions have been deleted.
    pub question_sticker_text: Mutable<Option<String>>,
    // Optional index of the currently open question.
    pub current_question: Mutable<Option<usize>>,
    pub text_editor: Rc<TextEditor>,
    pub play_settings: Rc<PlaySettings>,
    pub continue_next_fn: ContinueNextFn,
}

pub struct PlaySettings {
    pub ordering: Mutable<Ordering>,
    pub has_attempts_limit: Mutable<bool>,
    pub n_attempts: Mutable<u32>,
    pub has_time_limit: Mutable<bool>,
    pub time_limit: Mutable<u32>,
}

const DEFAULT_TIME_LIMIT: u32 = 20;

impl PlaySettings {
    pub fn new(settings: RawPlaySettings) -> Self {
        Self {
            ordering: Mutable::new(settings.ordering),
            has_attempts_limit: Mutable::new(settings.n_attempts.is_some()),
            n_attempts: Mutable::new(settings.n_attempts.unwrap_or(DEFAULT_ATTEMPTS_LIMIT)),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
            time_limit: Mutable::new(settings.time_limit.unwrap_or(DEFAULT_TIME_LIMIT)),
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

    /// Optional audio for incorrect choices
    pub incorrect_audio: Mutable<Option<Audio>>,

    /// Optional audio for correct choices
    pub correct_audio: Mutable<Option<Audio>>,

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
            incorrect_audio: Mutable::new(None),
            correct_audio: Mutable::new(None),
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
            incorrect_audio: Mutable::new(raw_question.incorrect_audio.clone()),
            correct_audio: Mutable::new(raw_question.correct_audio.clone()),
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
        let feedback = Mutable::new(content.base.feedback);

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

        let mut stickers_callbacks =
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
            })));

        stickers_callbacks.set_on_index_change(clone!(_self_ref, history => move |diff| {
            if let Some(_self) = _self_ref.borrow().as_ref() {
                if let QuestionField::Text(field_index) = _self.question_field.get_cloned() {
                    let modify_history = match diff {
                        VecDiff::RemoveAt { index } => {
                            if index == field_index {
                                // The sticker that was the field has been removed.
                                _self.question_field.set(QuestionField::Dynamic(None));
                                true
                            } else if index < field_index {
                                // The sticker that was removed is before the field index, so decrease the value.
                                _self.question_field.set(QuestionField::Text(field_index - 1));
                                true
                            } else {
                                false
                            }
                        }
                        VecDiff::Move { old_index, new_index } => {
                            if old_index == field_index {
                                // The sticker being moved is the question sticker.
                                _self.question_field.set(QuestionField::Text(new_index));
                                true
                            } else if old_index < field_index && new_index >= field_index {
                                // In the move operation, old_index is the index of the value _removed_, and new_index
                                // is the index of the value _inserted_.
                                //
                                // If the original index was before the field index, and the new index is the same or higher
                                //  than field index, decrease the value.
                                _self.question_field.set(QuestionField::Text(field_index - 1));
                                true
                            } else if old_index > field_index && new_index <= field_index {
                                // As above, but we need to shift the index up
                                _self.question_field.set(QuestionField::Text(field_index + 1));
                                true
                            } else {
                                false
                            }
                        }
                        _ => {
                            false
                        }
                    };

                    if modify_history {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.question_field = _self.question_field.get_cloned();
                            }
                        });
                    }
                }
            }
        }));

        let stickers = Stickers::new(text_editor.clone(), stickers_callbacks);

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
            feedback,
            text_editor,
            phase: Mutable::new(Phase::Layout),
            backgrounds,
            stickers,
            questions: Question::from_raw_vec(content.questions),
            question_field: Mutable::new(content.question_field),
            question_sticker_text: Mutable::new(None),
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

impl DesignExt<Mode> for Base {
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

    fn get_image_tag_priorities(&self) -> Option<Vec<ImageTag>> {
        let mode = self.history.get_current().mode();
        mode.map(|mode| match mode {
            Mode::Family => vec![ImageTag::PhotoAlbum],
            Mode::Map => vec![ImageTag::Map],
            Mode::MultipleChoice => vec![ImageTag::MultipleChoice],
            Mode::Scene => vec![],
            Mode::Text => vec![ImageTag::Boards, ImageTag::Book],
            Mode::Differences => vec![],
        })
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
