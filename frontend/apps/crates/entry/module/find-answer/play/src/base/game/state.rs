use crate::base::state::*;
use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::module::body::find_answer::Question;

pub struct Game {
    pub base: Rc<Base>,
    pub phase: Mutable<Phase>,
    /// Holds the current question and its index in the list of questions.
    pub question: Mutable<Option<(usize, Rc<Question>)>>,
}

impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        // let phase = Mutable::new(match base.settings.hint {
        //     Hint::Highlight => Phase::ShowHints,
        //     Hint::None => Phase::Playing,
        // });

        // TODO We need to choose random questions if that setting is enabled for the activity.
        // Fetch the first question. It should be guaranteed that at least one question always exists when playing.
        // However, there is always the possibility that a teacher wants to preview an incomplete activity which has
        // no questions yet, so we leave this value as optional.
        let first_question = base
            .questions
            .clone()
            .first()
            .cloned()
            .map(|question| (0, question));

        Rc::new(Self {
            base,
            phase: Mutable::new(Phase::Playing),
            question: Mutable::new(first_question),
        })
    }

    pub fn next_question_index(self: &Rc<Game>) -> Option<usize> {
        if let Some((current_index, ..)) = self.question.get_cloned() {
            let next_index = current_index + 1;
            if next_index < self.base.questions.len() {
                return Some(next_index);
            }
        }

        None
    }

    pub fn move_next_question(self: &Rc<Game>, index: usize) {
        if let Some(next_question) = self.base.questions.get(index) {
            self.question.set(Some((index, next_question.clone())));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Playing,
}
