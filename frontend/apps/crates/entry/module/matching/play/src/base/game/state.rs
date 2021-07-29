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
    pub deck: RefCell<Vec<CardPairId>>,
    pub current: Mutable<Option<Rc<Current>>>,
}

#[derive(Clone)]
pub struct CardPairId(pub Card, pub Card, pub usize);


#[derive(Clone)]
pub struct Current {
    pub top: Vec<Rc<CardTop>>,
    pub bottom: Vec<Rc<CardBottom>>,
    pub drag: Mutable<Option<Rc<CardDrag>>>,
}


impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let mut rng = thread_rng();
        let mut deck:Vec<CardPairId> = base.raw_pairs
            .iter()
            .enumerate()
            .map(|(index, pair)| {
                CardPairId (pair.0.clone(), pair.1.clone(), index)
            })
            .collect();
       
        deck.shuffle(&mut rng); 


        let _self = Rc::new(Self { 
            base,
            deck: RefCell::new(deck),
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

impl Current {
    pub fn new(game: Rc<Game>) -> Option<Rc<Self>> {
        let deck_len = game.deck.borrow().len();

        if deck_len < 2 {
            None
        } else {
            let amount:usize = game.base.settings.n_choices.into();
            let amount:usize = amount.min(deck_len);

            let mut top:Vec<Rc<CardTop>> = Vec::new();
            let mut bottom:Vec<Rc<CardBottom>> = Vec::new();

            let mut pairs:Vec<CardPairId> = game.deck.borrow_mut().drain(0..amount).collect();

            pairs.shuffle(&mut *game.rng.borrow_mut());

            for (choice_id, pair) in pairs.iter().enumerate() {
                top.push(Rc::new(CardChoice::<TopPhase>::new(game.clone(), pair.clone(), choice_id)));
            }

            pairs.shuffle(&mut *game.rng.borrow_mut());

            for (choice_id, pair) in pairs.iter().enumerate() {
                bottom.push(Rc::new(CardChoice::<BottomPhase>::new(game.clone(), pair.clone(), choice_id)));
            }


            Some(Rc::new(Self {
                top,
                bottom,
                drag: Mutable::new(None),
            }))
        }
    }
}

