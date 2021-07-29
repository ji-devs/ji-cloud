use super::state::*;
use components::module::_groups::cards::lookup::Side;
use gloo_timers::future::TimeoutFuture;
use shared::domain::jig::module::body::_groups::cards::{CardPair, Card};
use shared::domain::jig::module::body::card_quiz::PlayerSettings;
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

    pub fn check(&self, pair_id: usize) -> bool {
        pair_id == self.current.lock_ref().target.pair_id
    }

}

pub fn get_current(deck: &mut Vec<CardPairId>, mut rng: &mut ThreadRng, settings: &PlayerSettings) -> Option<Current> {
    log::info!("deck: {} pairs", deck.len());

    if deck.len() < 2 {
        None
    } else {
        deck.pop()
            .map(|pair| {

                let PlayerSettings { n_choices, swap, ..} = settings;

                //Get random "distractors"
                let mut others:Vec<CardPairId> = deck
                    .choose_multiple(&mut rng, (*n_choices-1).into())
                    .cloned()
                    .collect();
              
                //Add the correct option
                others.push(pair.clone());

                //re-shuffle
                others.shuffle(&mut rng);


                //Get our actual correct cards based on swap
                let target = {
                    if !swap {
                        pair.0
                    } else {
                        pair.1
                    }
                };

                let others = others
                    .into_iter()
                    .map(|pair| {
                        if !swap {
                            pair.1
                        } else {
                            pair.0
                        }
                    })
                    .collect();

                //needed for styling
                let side = {
                    if !swap {
                        Side::Left
                    } else {
                        Side::Right
                    }
                };

                Current {
                    target,
                    others,
                    side,
                    phase: Mutable::new(CurrentPhase::Waiting)
                }

            })
    }
}
