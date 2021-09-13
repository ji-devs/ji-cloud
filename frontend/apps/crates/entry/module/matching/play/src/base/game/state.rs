use components::module::_groups::cards::lookup::Side;
use shared::domain::jig::module::body::{
    _groups::cards::{CardPair, Card},
    matching::PlayerSettings
};
use std::sync::atomic::{AtomicUsize, Ordering};
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
    pub rounds_played: AtomicUsize,
}
impl Game {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let _self = Rc::new(Self { 
            base,
            remaining: RefCell::new(Vec::new()),
            used: RefCell::new(Vec::new()),
            rng: RefCell::new(thread_rng()),
            current: Mutable::new(None),
            rounds_played: AtomicUsize::new(0),
        });
        Self::reset_deck(_self.clone());
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

        //remove our target from potential choices
        let target = remaining.pop().unwrap_ji();

        //first get all the potential choices
        let mut others:Vec<&CardPairId> = remaining.iter().chain(used.iter()).collect();
        //shuffle them up
        others.shuffle(rng);

        //take just what we need
        let mut others:Vec<CardPairId> = others
            .into_iter()
            .take(amount-1)
            .map(|x| x.clone())
            .collect();

        //add in our target 
        others.push(target.clone());

        //re-shuffle to move it 
        others.shuffle(rng);
        
        //add the target to the used buffer
        used.push(target.clone());


        /// clone into top/bottom

        let mut top:Vec<Rc<CardTop>> = others
            .iter()
            .map(|choice| {
                Rc::new(CardChoice::<TopPhase>::new(game.clone(), choice.clone()))
            })
            .collect();

        let mut bottom:Vec<Rc<CardBottom>> = others
            .iter()
            .map(|choice| {
                Rc::new(CardChoice::<BottomPhase>::new(game.clone(), choice.clone()))
            })
            .collect();

        //shuffle again so there's some horizontal difference
        top.shuffle(rng);
        bottom.shuffle(rng);

        Rc::new(Self {
            top,
            bottom,
            drag: Mutable::new(None),
        })
    }
}
