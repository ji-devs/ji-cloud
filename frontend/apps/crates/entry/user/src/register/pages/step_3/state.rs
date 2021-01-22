use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step2Data};

pub struct State {
    pub step: Mutable<Step>,
    pub step_2: Step2Data
}

impl State {
    pub fn new(step: Mutable<Step>, step_2: Step2Data) -> Self {
        Self {
            step,
            step_2
        }
    }
}
