use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::module::_common::edit::prelude::*;
use futures_signals::signal::Mutable;
use js_sys::Reflect;
use shared::domain::module::body::{
    _groups::design::Trace,
    find_answer::{Mode, ModuleData as RawData, Question as RawQuestion, QuestionField, Step},
};
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::JsValue;

pub enum Direction {
    Up,
    Down,
}

pub async fn init_from_raw(
    init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
) -> BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {
    let force_step = {
        if init_args.source == InitSource::ForceRaw {
            crate::debug::settings().step
        } else {
            None
        }
    };

    let base = Base::new(init_args).await;

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base)),
    }
}

impl Base {
    pub fn set_current_question(&self, index: usize) {
        if let Some(current_index) = self.current_question.get_cloned() {
            if current_index == index {
                self.current_question.set_neq(None)
            } else {
                self.current_question.set_neq(Some(index))
            }
        } else {
            self.current_question.set_neq(Some(index))
        }
    }
    pub fn add_question(&self, question_text: Option<String>, question_field_index: Option<usize>) {
        self.questions.lock_mut().push_cloned(Rc::new(Question {
            question_text: Mutable::new(question_text.clone()),
            ..Default::default()
        }));

        if let Some(index) = question_field_index {
            self.question_field.set(QuestionField::Text(index));
        }

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                let question_text = question_text.unwrap_or_default();
                let raw_question = RawQuestion {
                    question_text,
                    ..Default::default()
                };
                content.questions.push(raw_question);
                content.question_field = self.question_field.get_cloned();
            }
        });
    }

    pub fn add_default_question(&self) {
        self.add_question(None, None);
    }

    pub fn move_question(&self, old_index: usize, direction: Direction) {
        let mut questions = self.questions.lock_mut();

        let new_index = match direction {
            Direction::Up => {
                if old_index == 0 {
                    return;
                }
                old_index - 1
            }
            Direction::Down => {
                if old_index >= questions.len() - 1 {
                    return;
                }
                old_index + 1
            }
        };

        questions.move_from_to(old_index, new_index);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                let item = content.questions.remove(old_index);
                content.questions.insert(new_index, item);
            }
        });

        self.current_question.set_neq(None);
    }

    pub fn delete_question(&self, index: usize) {
        self.questions.lock_mut().remove(index);
        if self.questions.lock_ref().is_empty() {
            if let QuestionField::Text(index) = self.question_field.get_cloned() {
                let text = self.stickers.get_as_text(index).unwrap_ji();
                text.can_delete.set_neq(true);

                if let Some(value) = self.question_sticker_text.get_cloned() {
                    Reflect::set(
                        &text.renderer_ref.get_cloned().unwrap_ji(),
                        &JsValue::from_str("textValue"),
                        &JsValue::from_str(&value),
                    )
                    .unwrap_ji();
                    Reflect::set(
                        &text.measurer_ref.get_cloned().unwrap_ji(),
                        &JsValue::from_str("textValue"),
                        &JsValue::from_str(&value),
                    )
                    .unwrap_ji();
                }
            }
        }

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.questions.remove(index);
                content.question_field = self.question_field.get_cloned();
            }
        });
    }

    pub fn save_question(&self, index: usize, question: Rc<Question>) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                let traces = match content.questions.get_mut(index) {
                    // If this is an existing question, we can just go ahead and use the
                    // traces already saved
                    Some(raw_question) => raw_question.traces.clone(),
                    // Otherwise, this question hasn't been saved yet, so we should first
                    // collect the traces that may have been added, and save those with the
                    // new question
                    None => question
                        .traces
                        .lock_ref()
                        .iter()
                        .map(|trace| trace.clone())
                        .collect(),
                };

                let raw_question = RawQuestion {
                    title: question.title.get_cloned().unwrap_or_default(),
                    question_text: question.question_text.get_cloned().unwrap_or_default(),
                    question_audio: question.question_audio.get_cloned(),
                    incorrect_audio: question.incorrect_audio.get_cloned(),
                    correct_audio: question.correct_audio.get_cloned(),
                    traces,
                };

                content.questions.remove(index);
                content.questions.insert(index, raw_question);
            }
        });
    }

    pub fn on_trace_added(&self, trace: Trace) {
        if let Some(current_question) = self.current_question.get_cloned() {
            if let Some(question) = self.questions.lock_ref().get(current_question) {
                question.traces.lock_mut().push_cloned(trace.clone());
            }

            self.history.maybe_push_modify(move |mut raw| {
                // Content may not exist yet
                if let Some(content) = &mut raw.content {
                    // A question may not exist yet and we don't want to fire of PATCH requests
                    // everytime the teacher adds a trace on a question which hasn't been saved
                    // yet.
                    if let Some(question) = content.questions.get_mut(current_question) {
                        question.traces.push(trace);
                        return Some(raw);
                    }
                }

                None
            });
        }
    }

    pub fn on_trace_deleted(&self, index: usize) {
        if let Some(current_question) = self.current_question.get_cloned() {
            if let Some(question) = self.questions.lock_ref().get(current_question) {
                question.traces.lock_mut().remove(index);
            }

            self.history.maybe_push_modify(move |mut raw| {
                if let Some(content) = &mut raw.content {
                    if let Some(question) = content.questions.get_mut(current_question) {
                        question.traces.remove(index);

                        return Some(raw);
                    }
                }

                None
            });
        }
    }

    pub fn on_trace_changed(&self, index: usize, raw_trace: Trace) {
        if let Some(current_question) = self.current_question.get_cloned() {
            if let Some(question) = self.questions.lock_ref().get(current_question) {
                let mut traces = question.traces.lock_mut();
                traces.remove(index);
                traces.insert_cloned(index, raw_trace.clone());
            }

            self.history.maybe_push_modify(move |mut raw| {
                if let Some(content) = &mut raw.content {
                    if let Some(question) = content.questions.get_mut(current_question) {
                        question.traces[index] = raw_trace;

                        return Some(raw);
                    }
                }

                None
            });
        }
    }
}
