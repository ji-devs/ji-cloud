use crate::base::state::Base;
use components::module::_groups::cards::lookup::Side;
use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::jig::module::body::_groups::cards::Card;
use shared::domain::jig::module::body::card_quiz::PlayerSettings;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use utils::prelude::*;

pub struct Game {
    pub base: Rc<Base>,
    pub rng: RefCell<ThreadRng>,
    pub remaining: RefCell<Vec<CardPairId>>,
    pub used: RefCell<Vec<CardPairId>>,
    pub current: Mutable<Option<Rc<Current>>>,
    pub rounds_played: AtomicUsize,
}

#[derive(Clone, Debug)]
pub struct CardPairId(pub CardId, pub CardId);

#[derive(Clone, Debug)]
pub struct CardId {
    pub card: Card,
    pub pair_id: usize,
}

pub struct Current {
    pub target: CardId,
    pub others: Vec<CardId>,
    pub incorrect_choices: RefCell<Vec<usize>>,
    pub side: Side,
    pub phase: Mutable<CurrentPhase>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrentPhase {
    Waiting,
    Correct(usize),
    Wrong(usize),
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
}

impl Current {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let PlayerSettings {
            n_choices, swap, ..
        } = game.base.settings;

        let remaining = &mut *game.remaining.borrow_mut();
        let used = &mut *game.used.borrow_mut();
        let rng = &mut *game.rng.borrow_mut();

        let _deck_len = remaining.len() + used.len();

        let amount: usize = n_choices.into();
        let amount: usize = amount.min(game.base.raw_pairs.len());

        // Remaining and used is split so that we can detect
        // when the entire deck has been looped through
        // which can be useful for educational purposes
        // even though it technically isn't in play settings yet

        //remove our target from potential choices
        let target = remaining.pop().unwrap_ji();

        //first get all the potential choices
        let mut others: Vec<&CardPairId> = remaining.iter().chain(used.iter()).collect();
        //shuffle them up
        others.shuffle(rng);

        //take just what we need
        let mut others: Vec<CardPairId> = others.into_iter().take(amount - 1).cloned().collect();

        //add in our target
        others.push(target.clone());

        //re-shuffle to move it
        others.shuffle(rng);

        //add the target to the used buffer
        used.push(target.clone());

        //reduce it down to just the sides

        let target = {
            if !swap {
                target.0
            } else {
                target.1
            }
        };

        let others = others
            .into_iter()
            .map(|pair| if !swap { pair.1 } else { pair.0 })
            .collect();

        //needed for styling
        let side = {
            if !swap {
                Side::Left
            } else {
                Side::Right
            }
        };
        Rc::new(Self {
            target,
            others,
            incorrect_choices: RefCell::new(Vec::new()),
            side,
            phase: Mutable::new(CurrentPhase::Waiting),
        })
    }
}
