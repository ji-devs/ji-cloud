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
use super::{
    drag::*,
    select::*
};

pub struct Main {
    pub base: Rc<Base>,
}

impl Main {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
        }
    }

    pub fn sticker_phase_signal(&self) -> impl Signal<Item = StickerPhase> {
        let base = self.base.clone();

        self.base.step.signal()
            .map(clone!(base => move |step| match step {
                Step::One => StickerPhase::Scene,
                Step::Two => StickerPhase::Select(MainSelect::new(base.clone())),
                Step::Four => StickerPhase::Drag(MainDrag::new(base.clone())),
                _ => StickerPhase::Static,
            }))
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

#[derive(Clone)]
pub enum StickerPhase {
    Scene,
    Select(Rc<MainSelect>),
    Drag(Rc<MainDrag>),
    Static,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TracePhase {
    Edit,
    Show,
}


impl MainExt for Main {
}

