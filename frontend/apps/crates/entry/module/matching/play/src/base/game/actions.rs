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
    pub fn next(&self) {
        if let Some(next) = get_current(&mut self.deck.borrow_mut(), &mut self.rng.borrow_mut(), &self.base.settings) {
            self.current.set(next);
        } else {
            self.base.phase.set(Phase::Ending(Rc::new(Ending::new(self.base.clone()))));
        }
    }

}

pub fn get_current(deck: &mut Vec<CardPairId>, mut rng: &mut ThreadRng, settings: &PlayerSettings) -> Option<Current> {
    log::info!("deck: {} pairs", deck.len());

    if deck.len() < 2 {
        None
    } else {
        let amount:usize = settings.n_choices.into();
        let amount:usize = amount.min(deck.len());

        let PlayerSettings { n_choices, swap, ..} = settings;

        let mut top:Vec<CardId> = Vec::new();
        let mut bottom:Vec<CardId> = Vec::new();

        for pair in deck.drain(0..amount) {
            if !swap {
                top.push(pair.0);
                bottom.push(pair.1);
            } else {
                top.push(pair.1);
                bottom.push(pair.0);
            }
        }

        top.shuffle(&mut rng);
        bottom.shuffle(&mut rng);

        //needed for styling
        let side = {
            if !swap {
                Side::Left
            } else {
                Side::Right
            }
        };

        Some(Current {
            top,
            bottom,
            side,
            phase: Mutable::new(CurrentPhase::Waiting)
        })

    }
}
