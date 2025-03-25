use crate::base::game::state::*;
use components::traces::{bubble::TraceBubble, utils::TraceExt};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::module::body::_groups::design::Trace;
use std::{ops::Deref, rc::Rc};

use std::collections::HashSet;

pub struct PlayState {
    pub game: Rc<Game>,
    pub traces: Vec<Rc<PlayTrace>>,
    /// Used for highlighting the last selected trace.
    pub current_set: Mutable<HashSet<usize>>,
    pub selected_set: Mutable<HashSet<usize>>,
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let traces = game
            .base
            .traces
            .iter()
            .map(|trace| PlayTrace::new(trace.clone()))
            .collect();

        Rc::new(Self {
            game,
            traces,
            current_set: Mutable::new(HashSet::new()),
            selected_set: Mutable::new(HashSet::new()),
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

    pub fn select(&self, play_state: Rc<PlayState>) {
        if self.audio.is_none() && self.text.is_none() {
            self.phase.set(PlayPhase::IdleSelected);
        } else if let Some(bounds) = self.inner.calc_bounds(true) {
            let bubble = TraceBubble::new(
                bounds,
                self.audio.clone(),
                self.text.clone(),
                Some(clone!(play_state => move || {
                    // Clear the current trace highlight if it is set
                    play_state.current_set.lock_mut().clear();
                    play_state.evaluate_end();
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
