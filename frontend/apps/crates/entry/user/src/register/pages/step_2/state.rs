use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step2Data};

pub struct State {
    pub step: Mutable<Step>,
    pub init_data: Step2Data,
}

impl State {
    pub fn new(step: Mutable<Step>, init_data: Step2Data) -> Self {
        Self {
            step,
            init_data
        }
    }
}
