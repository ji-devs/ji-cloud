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
use crate::base::{
    state::Phase,
    ending::state::Ending
};

impl Game {
    pub fn next(_self: Rc<Self>) {
        _self.current.set(Current::new(_self.clone()));

        if _self.current.lock_ref().is_none() {
            _self.base.phase.set(Phase::Ending(Rc::new(Ending::new(_self.base.clone()))));
        }
    }
}
