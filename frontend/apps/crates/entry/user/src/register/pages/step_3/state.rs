use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::Step;

pub struct State {
    pub step: Mutable<Step>
}

impl State {
    pub fn new(step: Mutable<Step>) -> Self {
        Self {
            step
        }
    }
}
