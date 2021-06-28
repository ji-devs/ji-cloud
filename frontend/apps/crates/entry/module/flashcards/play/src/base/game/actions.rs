use super::state::*;
use components::module::_groups::cards::lookup::Side;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::{_groups::cards::{CardPair, Card}, flashcards::DisplayMode};
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
    pub fn next(&self) {
        if let Some(next) = get_current(&mut self.deck.borrow_mut(), &mut self.rng.borrow_mut()) {
            self.current.set(next);
        } else {
            self.base.phase.set(Phase::Ending(Rc::new(Ending::new(self.base.clone()))));
        }
    }

}

pub fn get_current(deck: &mut Vec<CardPair>, rng: &mut ThreadRng) -> Option<Current> {
    log::info!("deck: {} pairs", deck.len());

    deck.pop()
        .map(|pair| {
   
            if rng.gen::<bool>() { 
                Current {
                    card: pair.0,
                    other: pair.1,
                    side: Side::Left 
                }
            } else { 
                Current {
                    card: pair.1,
                    other: pair.0,
                    side: Side::Right
                }
            }
        })
}
