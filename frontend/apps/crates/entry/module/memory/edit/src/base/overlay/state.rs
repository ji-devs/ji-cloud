use components::module::edit::*;
use utils::unwrap::UnwrapJiExt;
use super::super::state::Base;
use std::rc::Rc;
use components::tooltip::state::State as TooltipState;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
pub struct Overlay {
    pub base: Rc<Base>
}


impl Overlay {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base 
        }
    }
}

impl OverlayExt for Overlay {
}
