use components::module::edit::*;
use super::super::state::{Step, Base};
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;

pub struct Sidebar {
    pub base: Rc<Base>,
    pub step_reactor: AsyncLoader,
    pub step_state: Mutable<Option<Rc<StepState>>>
}


impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        let step_reactor = AsyncLoader::new();

        let step_state = Mutable::new(None);

        step_reactor.load(base.step.signal().for_each(clone!(step_state => move |step| {
            match step {
                Step::One => {
                    step_state.set(Some(Rc::new(StepState::One)));
                },
                Step::Two => {
                    step_state.set(Some(Rc::new(StepState::Two)));
                },
                Step::Three => {
                    step_state.set(Some(Rc::new(StepState::Three)));
                },
                _ => {
                    step_state.set(None);
                }
            }
            async {}
        })));

        Self {
            base,
            step_reactor,
            step_state,
        }
    }
}

impl SidebarExt for Sidebar {
}


//these will be much heavier...
pub enum StepState {
    One,
    Two,
    Three
}
