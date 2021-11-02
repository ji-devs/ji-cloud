use crate::state::*;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt},
};

use components::module::_groups::cards::lookup::Side;
use rand::prelude::*;
use shared::domain::jig::module::body::_groups::cards::Card;
use std::rc::Rc;

pub struct MainSettings {
    pub base: Rc<Base>,
    pub pairs: Rc<Vec<(Card, Card)>>,
}

//pub pairs: MutableVec<(Card, Card)>,
impl MainSettings {
    pub fn new(base: Rc<Base>) -> Self {
        let _settings = &base.extra.settings;

        let mut pairs = base.clone_pairs_raw();

        pairs.shuffle(&mut *base.extra.settings.rng.borrow_mut());

        Self {
            base,
            pairs: Rc::new(pairs),
        }
    }

    pub fn top_choices_signal(&self) -> impl Signal<Item = Vec<(Card, Side)>> {
        self.choices_signal(self.base.extra.settings.swap.signal().map(|swap| {
            if swap {
                Side::Right
            } else {
                Side::Left
            }
        }))
    }

    pub fn bottom_choices_signal(&self) -> impl Signal<Item = Vec<(Card, Side)>> {
        self.choices_signal(self.base.extra.settings.swap.signal().map(|swap| {
            if swap {
                Side::Left
            } else {
                Side::Right
            }
        }))
    }

    fn choices_signal(
        &self,
        side_signal: impl Signal<Item = Side>,
    ) -> impl Signal<Item = Vec<(Card, Side)>> {
        let pairs = self.pairs.clone();

        let sig = map_ref! {
            let side = side_signal,
            let n_choices = self.base.extra.settings.n_choices.signal()
                => (*side, *n_choices)
        };

        sig.map(move |(side, n_choices)| {
            pairs
                .iter()
                .take(n_choices.into())
                .map(|pair| {
                    let card = {
                        if side == Side::Left {
                            pair.0.clone()
                        } else {
                            pair.1.clone()
                        }
                    };

                    (card, side)
                })
                .collect::<Vec<(Card, Side)>>()
        })
    }
}
