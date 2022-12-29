use crate::base::game::state::*;
use components::traces::{bubble::TraceBubble, utils::TraceExt};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::_groups::design::Trace;
use shared::domain::module::body::find_answer::Question;
use std::{ops::Deref, rc::Rc};

use std::collections::HashSet;

pub struct PlayState {
    /// Base game state
    pub game: Rc<Game>,
    /// The current question.
    ///
    /// We *do* know what the current question is already via `Game` state, however, that field is
    /// optional, and this field is guaranteed to always have a current question, so we don't
    /// need to do any further checks when rendering/playing.
    pub question: Rc<Question>,
    /// Traces mapped from the raw trace list in `question`.
    pub traces: Vec<Rc<PlayTrace>>,
    /// Set of traces already selected by the student.
    pub selected_set: Mutable<HashSet<usize>>,
    /// Flag indicating whether the question has already ended. When true, subsequent taps on the answer traces will
    /// not trigger any logic.
    pub ended: Mutable<bool>,
    /// Number of times the student has selected the incorrect choice.
    pub incorrect_choice_count: Mutable<u32>,
    /// Whether trace hints should be shown.
    pub show_hint: Mutable<bool>,
}

impl PlayState {
    pub fn new(game: Rc<Game>, question: Rc<Question>) -> Rc<Self> {
        let traces = question
            .traces
            .iter()
            .map(|trace| PlayTrace::new(game.clone(), trace.clone()))
            .collect();

        Rc::new(Self {
            game,
            question,
            traces,
            selected_set: Mutable::new(HashSet::new()),
            ended: Mutable::new(false),
            incorrect_choice_count: Mutable::new(0),
            show_hint: Mutable::new(false),
        })
    }
}

pub struct PlayTrace {
    pub game: Rc<Game>,
    pub phase: Mutable<PlayPhase>,
    pub inner: Trace,
}

impl Deref for PlayTrace {
    type Target = Trace;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PlayTrace {
    pub fn new(game: Rc<Game>, trace: Trace) -> Rc<Self> {
        Rc::new(Self {
            game,
            phase: Mutable::new(PlayPhase::Waiting),
            inner: trace,
        })
    }

    pub fn select(&self, play_state: Rc<PlayState>) {
        if self.audio.is_none() && self.text.is_none() {
            self.phase.set(PlayPhase::IdleSelected);
            play_state.evaluate_end();
        } else if let Some(bounds) = self.inner.calc_bounds(true) {
            let bubble = TraceBubble::new(
                bounds,
                self.audio.clone(),
                self.text.clone(),
                Some(clone!(play_state => move || {
                    play_state.clone().evaluate_end();
                })),
            );
            self.phase.set(PlayPhase::Playing(bubble));
        }
    }

    pub fn kill_playback(&self) {
        self.phase.replace_with(|curr| match curr {
            PlayPhase::Waiting => PlayPhase::Waiting,
            _ => PlayPhase::IdleSelected,
        });
    }
}

#[derive(Clone)]
pub enum PlayPhase {
    Waiting,
    Playing(Rc<TraceBubble>),
    IdleSelected,
}
