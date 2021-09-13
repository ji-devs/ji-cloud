use super::state::*;
use components::module::_groups::cards::lookup::Side;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::_groups::cards::{CardPair, Card};
use shared::domain::jig::module::body::matching::PlayerSettings;
use wasm_bindgen_futures::spawn_local;
use crate::base::state::Base;
use std::cell::RefCell;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use rand::prelude::*;
use utils::prelude::*;
use crate::base::state::Phase;
use components::module::_common::play::prelude::*;
use std::convert::TryInto;

impl Game {
    pub fn next(_self: Rc<Self>) {
        let has_ended = {
            if _self.used.borrow().len() >= _self.base.settings.n_rounds.try_into().unwrap_ji() {
                true
            } else if _self.remaining.borrow().len() == 0 {
                //DECK FINISHED! (this isn't supported in settings *yet*)
                //should be very easy to just reset used/remaining with new deck
                true
            } else {
                false
            }
        };

        if !has_ended {
            _self.current.set(Some(Current::new(_self.clone())));
        } else {
            _self.base.phase.set(Phase::Ending);
            _self.base.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Positive)));
        }
    }
}
