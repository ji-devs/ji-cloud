use components::module::_groups::cards::lookup::Side;
use shared::domain::jig::module::body::{
    _groups::cards::{CardPair, Card},
    matching::PlayerSettings
};
use crate::base::state::Base;
use std::cell::RefCell;
use std::rc::Rc;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt}
};
use rand::prelude::*;
use utils::prelude::*;
use super::card::state::*;

pub struct Game {
    pub base: Rc<Base>,
    pub rng: RefCell<ThreadRng>,
    pub remaining: RefCell<Vec<CardPairId>>,
    pub used: RefCell<Vec<CardPairId>>,
    pub current: Mutable<Option<Rc<Current>>>,
}
impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let mut rng = thread_rng();

        let remaining = get_new_deck(&base, &mut rng);
        let used = Vec::with_capacity(remaining.len());

        let _self = Rc::new(Self { 
            base,
            remaining: RefCell::new(remaining),
            used: RefCell::new(used),
            rng: RefCell::new(rng),
            current: Mutable::new(None),
        });

        Self::next(_self.clone());

        _self
    }

    pub fn get_current(&self) -> Option<Rc<Current>> {
        self.current.get_cloned()
    }
}

#[derive(Clone, Debug)]
pub struct CardPairId(pub Card, pub Card, pub usize);


#[derive(Clone)]
pub struct Current {
    pub top: Vec<Rc<CardTop>>,
    pub bottom: Vec<Rc<CardBottom>>,
    pub drag: Mutable<Option<Rc<CardDrag>>>,
}

impl Current {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let remaining = &mut *game.remaining.borrow_mut();
        let used = &mut *game.used.borrow_mut();
        let rng = &mut *game.rng.borrow_mut();

        let deck_len = (remaining.len() + used.len());

        let amount:usize = game.base.settings.n_choices.into();
        let amount:usize = amount.min(game.base.raw_pairs.len());

        // Remaining and used is split so that we can detect
        // when the entire deck has been looped through
        // which can be useful for educational purposes
        // even though it technically isn't in play settings yet
        let mut top:Vec<Rc<CardTop>> = Vec::new();
        let mut bottom:Vec<Rc<CardBottom>> = Vec::new();

        let choice = remaining.pop().unwrap_ji();

        top.push(Rc::new(CardChoice::<TopPhase>::new(game.clone(), choice.clone())));
        bottom.push(Rc::new(CardChoice::<BottomPhase>::new(game.clone(), choice.clone())));

        for _ in 0..amount-1 {
            let pair = {
                let (a, b) = if rng.gen_bool(0.5) {
                    (&remaining, &used)
                } else {
                    (&used, &remaining)
                };

                let a_len = a.len();
                let b_len = b.len();

                let i = if a_len > 0 { rng.gen_range(0..a.len()) } else { 0 };

                match a.get(i) {
                    Some(pair) => pair,
                    None => {
                        let i = if b_len > 0 { rng.gen_range(0..b.len()) } else { 0 };
                        match b.get(i) {
                            Some(pair) => pair,
                            None => {
                                panic!("couldn't get a pair!");
                            }
                        }
                    }
                }
            };

            top.push(Rc::new(CardChoice::<TopPhase>::new(game.clone(), pair.clone())));
            bottom.push(Rc::new(CardChoice::<BottomPhase>::new(game.clone(), pair.clone())));
        }

        //why not
        top.shuffle(rng);
        bottom.shuffle(rng);

        used.push(choice);

        Rc::new(Self {
            top,
            bottom,
            drag: Mutable::new(None),
        })
    }
}


fn get_new_deck(base:&Base, rng: &mut ThreadRng) -> Vec<CardPairId> {
    let mut deck:Vec<CardPairId> = base.raw_pairs
        .iter()
        .enumerate()
        .map(|(index, pair)| {
            CardPairId (pair.0.clone(), pair.1.clone(), index)
        })
        .collect();
    
    deck.shuffle(rng); 

    deck
}