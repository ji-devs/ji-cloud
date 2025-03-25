use crate::base::game::state::*;
use components::audio::mixer::AudioHandle;
use components::traces::{bubble::TraceBubble, utils::TraceExt};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::Audio;
use shared::domain::module::body::_groups::design::Trace;
use shared::domain::module::body::find_answer::Question;
use std::cell::RefCell;
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
    /// Audio to play for the selected trace/incorrect choice.
    ///
    /// This is necessary to prevent an issue where playing audio from within AUDIO_MIXER.with(..) causes the
    /// `ENDED_CALLBACKS` RefCell to be mutably locked multiple times.
    pub selection_audio: Mutable<Option<Audio>>,
    /// Audio handle for currently playing choice. Required so that we can end the audio if the student clicks
    /// another answer while audio is already playing.
    pub selection_audio_handle: RefCell<Option<AudioHandle>>,
}

impl PlayState {
    pub fn new(game: Rc<Game>, question: Rc<Question>) -> Rc<Self> {
        let traces = question
            .traces
            .iter()
            .map(|trace| PlayTrace::new(trace.clone()))
            .collect();

        Rc::new(Self {
            game,
            question,
            traces,
            selected_set: Mutable::new(HashSet::new()),
            ended: Mutable::new(false),
            incorrect_choice_count: Mutable::new(0),
            show_hint: Mutable::new(false),
            selection_audio: Mutable::new(None),
            selection_audio_handle: RefCell::new(None),
        })
    }
}

pub struct PlayTrace {
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
    pub fn new(trace: Trace) -> Rc<Self> {
        Rc::new(Self {
            phase: Mutable::new(PlayPhase::Waiting),
            inner: trace,
        })
    }

    pub fn select(&self, play_state: Rc<PlayState>, on_ended: Option<fn(state: &Rc<PlayState>)>) {
        if self.audio.is_none() && self.text.is_none() {
            self.phase.set(PlayPhase::IdleSelected);
        } else if let Some(bounds) = self.inner.calc_bounds(true) {
            let phase = self.phase.clone();
            let bubble = TraceBubble::new(
                bounds,
                self.audio.clone(),
                self.text.clone(),
                Some(clone!(phase, play_state => move || {
                    play_state.clone().remove_incorrect_highlights();
                    if let Some(on_ended) = on_ended {
                        on_ended(&play_state)
                    }
                    phase.set(PlayPhase::IdleSelected);
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
