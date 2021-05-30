use components::module::edit::MainExt;
use components::traces::{
    bubble::state::TraceBubble,
    edit::state::Edit as TracesEdit
};
use crate::steps::state::{Step, Base};
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{SignalVec, SignalVecExt}
};
use utils::prelude::*;
use dominator::clone;

pub struct Main {
    pub base: Rc<Base>,
    pub step_reactor: AsyncLoader,
    pub phase: Mutable<Phase>
}

impl Main {
    pub fn new(base: Rc<Base>) -> Self {
        let step_reactor = AsyncLoader::new();

        let phase = Mutable::new(Phase::Layout);

        step_reactor.load(base.step.signal().for_each(clone!(base, phase => move |step| {
            if step == Step::Three {
                phase.set(Phase::Trace);
            } else {
                phase.set(Phase::Layout);
            }
            async {}
        })));

        Self {
            base,
            step_reactor,
            phase,
        }
    }

    pub fn trace_bubbles(&self) -> impl SignalVec<Item = Rc<TraceBubble>> {
        self.base
            .traces_meta
            .signal_vec_cloned()
            .map_signal(|trace_meta| trace_meta.bubble.signal_cloned())
            .filter(|bubble| bubble.is_some())
            .map(|bubble| bubble.unwrap_ji())
    }
}

#[derive(Clone)]
pub enum Phase {
    Layout,
    Trace
}


impl MainExt for Main {
}


