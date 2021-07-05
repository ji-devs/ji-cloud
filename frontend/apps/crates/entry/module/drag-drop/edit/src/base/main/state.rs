use components::module::_common::edit::prelude::*;
use components::traces::{
    bubble::state::TraceBubble,
    edit::state::Edit as TracesEdit
};
use crate::base::state::Base;
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{SignalVec, SignalVecExt}
};
use utils::prelude::*;
use dominator::clone;
use shared::domain::jig::module::body::drag_drop::Step;

pub struct Main {
    pub base: Rc<Base>,
}

impl Main {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }

    pub fn locked_scene_signal(&self) -> impl Signal<Item = bool> {
        self.base.step.signal()
            .map(|step| step != Step::One)
            .dedupe()
    }
    pub fn locked_drags_signal(&self) -> impl Signal<Item = bool> {
        self.base.step.signal()
            .map(|step| step != Step::Two)
            .dedupe()
    }
    pub fn trace_phase_signal(&self) -> impl Signal<Item = Option<TracePhase>> {
        self.base.step.signal()
            .map(|step| match step {
                Step::Three => Some(TracePhase::Edit),
                Step::Four => Some(TracePhase::Show),
                _ => None
            })
            .dedupe()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TracePhase {
    Edit,
    Show,
}


impl MainExt for Main {
}

