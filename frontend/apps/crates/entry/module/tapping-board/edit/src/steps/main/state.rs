use components::module::edit::MainExt;
use components::traces::edit::state::Edit as TracesEdit;
use crate::steps::state::{Step, Base};
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, SignalExt};
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
                phase.set(Phase::Trace(base.traces.clone()));
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
}

#[derive(Clone)]
pub enum Phase {
    Layout,
    Trace(Rc<TracesEdit>)
}


impl MainExt for Main {
}


