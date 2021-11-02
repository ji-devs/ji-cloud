use crate::state::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
    signal_vec::SignalVec,
};

use components::module::_groups::cards::lookup::Side;
use rand::prelude::*;
use shared::domain::jig::module::body::_groups::cards::Card;
use std::rc::Rc;

pub struct MainSettings {
    pub base: Rc<Base>,
    pub correct: Rc<(Card, Card)>,
    pub wrong_bank: Rc<Vec<(Card, Card)>>,
}

//pub pairs: MutableVec<(Card, Card)>,
impl MainSettings {
    pub fn new(base: Rc<Base>) -> Self {
        let _settings = &base.extra.settings;

        let mut pairs = base.clone_pairs_raw();

        let correct = pairs.remove(0); //TODO randomize this index in play mode

        pairs.shuffle(&mut *base.extra.settings.rng.borrow_mut());

        Self {
            base,
            correct: Rc::new(correct),
            wrong_bank: Rc::new(pairs),
        }
    }

    pub fn top_side_signal(&self) -> impl Signal<Item = Side> {
        self.base.extra.settings.swap.signal().map(
            |swap| {
                if swap {
                    Side::Right
                } else {
                    Side::Left
                }
            },
        )
    }

    pub fn correct_signal(&self) -> impl Signal<Item = (Card, Side)> {
        let correct = self.correct.clone();

        self.top_side_signal().map(move |top_side| {
            if top_side == Side::Left {
                (correct.0.clone(), top_side)
            } else {
                (correct.1.clone(), top_side)
            }
        })
    }

    pub fn choices_signal(&self) -> impl SignalVec<Item = (Card, Side, bool)> {
        let wrong_bank = self.wrong_bank.clone();
        let correct = self.correct.clone();

        let sig = map_ref! {
            let n_choices = self.base.extra.settings.n_choices.signal(),
            let top_side = self.top_side_signal()
                => (*n_choices, *top_side)
        };

        let rng = self.base.extra.settings.rng.clone();

        sig.map(move |(n_choices, top_side)| {
            let n_choices = n_choices.max(1); //just a safety precaution

            let bottom_side = top_side.negate();

            let mut choices: Vec<(Card, Side, bool)> = wrong_bank
                .iter()
                .take((n_choices - 1).into())
                .map(|pair| {
                    let card = if bottom_side == Side::Left {
                        &pair.0
                    } else {
                        &pair.1
                    };
                    (card.clone(), bottom_side, false)
                })
                .collect();

            let correct = if bottom_side == Side::Left {
                &correct.0
            } else {
                &correct.1
            };

            choices.push((correct.clone(), bottom_side, true));

            choices.shuffle(&mut *rng.borrow_mut());

            choices
        })
        .to_signal_vec()
    }
}
