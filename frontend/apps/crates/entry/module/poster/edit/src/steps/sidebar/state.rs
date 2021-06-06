use components::module::edit::prelude::*;
use crate::steps::state::{Step, Base};
use std::rc::Rc;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;
use super::{
    step_1::state::Step1,
    step_2::state::Step2,
    step_3::state::Step3,
};

pub struct Sidebar {
    pub base: Rc<Base>,
    pub step_reactor: AsyncLoader,
    pub step_state: Mutable<Option<StepState>>
}


impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        let step_reactor = AsyncLoader::new();

        let step_state = Mutable::new(None);

        step_reactor.load(base.step.signal().for_each(clone!(base, step_state => move |step| {
            match step {
                Step::One => {
                    step_state.set(Some(StepState::One(
                        Rc::new(Step1::new(base.clone()))
                    )));
                },
                Step::Two => {
                    step_state.set(Some(StepState::Two(
                        Rc::new(Step2::new(base.clone()))
                    )));
                },
                Step::Three => {
                    step_state.set(Some(StepState::Three(
                        Rc::new(Step3::new(base.clone()))
                    )));
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

#[derive(Clone)]
pub enum StepState {
    One(Rc<Step1>),
    Two(Rc<Step2>),
    Three(Rc<Step3>),
}
