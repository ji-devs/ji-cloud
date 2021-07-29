use components::module::_groups::cards::lookup::Side;
use shared::domain::jig::module::body::{_groups::cards::{CardPair, Card}, flashcards::DisplayMode};
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
    pub deck: RefCell<Vec<CardPair>>,
    pub current: Mutable<Current>,
}


#[derive(Clone)]
pub struct Current {
    pub card: Card,
    pub other: Card,
    pub side: Side, 
}

impl Game {
    pub fn new(base: Rc<Base>) -> Self {
        let mut rng = thread_rng();
        let mut deck = base.raw_pairs.clone();
       
        deck.shuffle(&mut rng); 

        let current = get_current(&mut deck, &mut rng).unwrap_ji();

        Self { 
            base,
            deck: RefCell::new(deck),
            rng: RefCell::new(rng),
            current: Mutable::new(current),
        }
    }
}

