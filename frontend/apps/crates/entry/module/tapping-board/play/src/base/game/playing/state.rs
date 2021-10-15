use crate::base::game::state::*;
use components::traces::{bubble::TraceBubble, utils::TraceExt};
use dominator::clone;
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::jig::module::body::_groups::design::Trace;
use std::{ops::Deref, rc::Rc};

use std::collections::HashSet;

pub struct PlayState {
    pub game: Rc<Game>,
    pub traces: Vec<Rc<PlayTrace>>,
    pub selected_set: Mutable<HashSet<usize>>,
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let traces = game
            .base
            .traces
            .iter()
            .map(|trace| PlayTrace::new(game.clone(), trace.clone()))
            .collect();

        Rc::new(Self {
            game,
            traces,
            selected_set: Mutable::new(HashSet::new()),
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
        } else {
            if let Some(bounds) = self.inner.calc_bounds(true) {
                let bubble = TraceBubble::new(
                    bounds,
                    self.audio.clone(),
                    self.text.clone(),
                    Some(clone!(play_state => move || {
                        play_state.evaluate_end();
                    })),
                );
                self.phase.set(PlayPhase::Playing(bubble));
            }
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
