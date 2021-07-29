use crate::base::state::*;
use std::rc::Rc;
use gloo_timers::future::TimeoutFuture;
use futures_signals::{
    map_ref,
    signal_vec::SignalVecExt,
    signal::{Mutable, Signal, SignalExt}
};
use shared::domain::jig::module::body::tapping_board::*;
use super::playing::state::*;

pub struct Game {
    pub base: Rc<Base>,
    pub phase: Mutable<Phase>
}

impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let phase = Mutable::new(match base.settings.hint {
            Hint::Highlight => Phase::ShowHints,
            Hint::None => Phase::Playing
        });

        let _self = Rc::new(Self {
            base,
            phase,
        });

        _self
    }


}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    ShowHints,
    Playing,
}

