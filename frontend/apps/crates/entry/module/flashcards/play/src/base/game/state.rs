use super::actions::*;
use crate::base::state::Base;
use components::module::_groups::cards::lookup::Side;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::jig::module::body::_groups::cards::{Card, CardPair};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;

pub struct Game {
    pub base: Rc<Base>,
    pub rng: RefCell<ThreadRng>,
    pub deck: RefCell<Vec<CardPair>>,
    pub current: Mutable<Current>,
    pub gate: Mutable<Gate>,
    pub animation_loader: AsyncLoader,
}

#[derive(Clone)]
pub struct Current {
    pub card: Card,
    pub other: Card,
    pub side: Side,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gate {
    Waiting,
    Flipping,
    FinishingFlip,
}
impl Game {
    pub fn new(base: Rc<Base>) -> Self {
        let mut rng = thread_rng();

        let mut deck = get_fresh_deck(&base, &mut rng);

        let current = get_current(&base, &mut deck).unwrap_ji();

        Self {
            base,
            deck: RefCell::new(deck),
            rng: RefCell::new(rng),
            current: Mutable::new(current),
            gate: Mutable::new(Gate::Waiting),
            animation_loader: AsyncLoader::new(),
        }
    }
}
