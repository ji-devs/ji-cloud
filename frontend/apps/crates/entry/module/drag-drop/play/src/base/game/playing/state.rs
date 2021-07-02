use std::{rc::Rc, cell::RefCell};
use crate::base::game::state::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, Mutable}
};
use components::traces::{
    utils::TraceExt,
    bubble::state::TraceBubble,
};
use shared::domain::jig::module::body::{
    Audio,
    _groups::design::Trace,
    drag_drop::{Next, DragDropTrace}
};
use web_sys::AudioContext;
use std::collections::HashSet;

pub struct PlayState {
    pub game: Rc<Game>,
    pub traces: Vec<Rc<PlayTrace>>,
    pub selected_set: RefCell<HashSet<usize>>
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {

            let traces =
                game.base.traces
                    .iter()
                    .map(|trace| PlayTrace::new(game.clone(), trace.clone()))
                    .collect();

        Rc::new(Self {
            game,
            traces,
            selected_set: RefCell::new(HashSet::new())
        })
    }

    pub fn select(&self, index: usize) {

        for (trace_index, trace) in self.traces.iter().enumerate() {
            if trace_index == index {
                trace.select();
            } else {
                trace.kill_playback();
            }
        }
        // mark the selected set
        let mut selected_set = self.selected_set.borrow_mut();

        selected_set.insert(index);

        let n_selected = selected_set.len();

        log::info!("{} SELECTED!", n_selected);

        let n_target = {
            match self.game.base.settings.next {
                Next::SelectAll => Some(self.traces.len()),
                Next::SelectSome(n) => Some(n),
                Next::Continue => None
            }
        };

        if let Some(n_target) = n_target {
            if n_selected >= n_target {
                log::warn!("TODO: GOING TO NEXT MODULE!");
            }
        }
    }
}

pub struct PlayTrace {
    pub game: Rc<Game>,
    pub phase: Mutable<PlayPhase>,
    pub inner: Trace,
    pub audio: Option<Audio>,
    pub text: Option<String>,
}

impl PlayTrace {
    pub fn new(game: Rc<Game>, tapping_trace: DragDropTrace) -> Rc<Self> {
        Rc::new(Self {
            game,
            phase: Mutable::new(PlayPhase::Waiting),
            inner: tapping_trace.trace,
            audio: tapping_trace.audio,
            text: tapping_trace.text
        })
    }

    pub fn select(&self) {
        if self.audio.is_none() && self.text.is_none() {
            self.phase.set(PlayPhase::IdleSelected);
        } else {
            if let Some(bounds)  = self.inner.calc_bounds(true) {
                let bubble = Rc::new(TraceBubble::new(bounds, self.audio.clone(), self.text.clone(), None::<fn()>));
                self.phase.set(PlayPhase::Playing(bubble));
            }
        }
    }

    pub fn kill_playback(&self) {
        self.phase.replace_with(|curr| match curr {
            PlayPhase::Waiting => PlayPhase::Waiting,
            _ => PlayPhase::IdleSelected
        });
    }
}

#[derive(Clone)]
pub enum PlayPhase {
    Waiting,
    Playing(Rc<TraceBubble>),
    IdleSelected
}
