use components::module::_groups::cards::lookup::Side;
use shared::domain::jig::module::body::_groups::cards::{CardPair, Card};
use crate::base::state::Base;
use std::cell::RefCell;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use rand::prelude::*;
use utils::prelude::*;
use super::actions::get_current;

pub struct Game {
    pub base: Rc<Base>,
    pub rng: RefCell<ThreadRng>,
    pub deck: RefCell<Vec<CardPairId>>,
    pub current: Mutable<Current>,
}


#[derive(Clone)]
pub struct CardPairId(pub CardId, pub CardId);

#[derive(Clone)]
pub struct CardId {
    pub card: Card,
    pub pair_id: usize
}

#[derive(Clone)]
pub struct Current {
    pub top: Vec<CardId>,
    pub bottom: Vec<CardId>,
    pub side: Side,
    pub phase: Mutable<CurrentPhase>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentPhase {
    Waiting,
    Dragging(usize),
    Correct(usize),
    Wrong(usize),
}

impl Game {
    pub fn new(base: Rc<Base>) -> Self {
        let mut rng = thread_rng();
        let mut deck:Vec<CardPairId> = base.raw_pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| {
                CardPairId (
                    CardId {
                        card: pair.0.clone(),
                        pair_id: index
                    },
                    CardId {
                        card: pair.1.clone(),
                        pair_id: index
                    }
                )
            })
            .collect();
       
        deck.shuffle(&mut rng); 

        let current = get_current(&mut deck, &mut rng, &base.settings).unwrap_ji();

        Self { 
            base,
            deck: RefCell::new(deck),
            rng: RefCell::new(rng),
            current: Mutable::new(current),
        }
    }
}

