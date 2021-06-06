use components::module::edit::prelude::*;
use dominator::{html, Dom, clone};
use std::rc::Rc;
use components::{backgrounds, stickers, traces};
use futures_signals::{signal::{ReadOnlyMutable, SignalExt}, signal_vec::SignalVecExt};
use crate::base::{
    state::*,
    main::state::Main
};
use super::card::state::*;

pub struct MainPair {
    pub base: Rc<Base>,
    pub step: Step, 
    pub index: ReadOnlyMutable<Option<usize>>,
    pub left: Rc<MainCard>,
    pub right: Rc<MainCard>
}

impl MainPair {
    pub fn new(
        main: Rc<Main>, 
        step: Step, 
        index: ReadOnlyMutable<Option<usize>>,
        pair: (Card, Card)
    ) -> Rc<Self> {
        Rc::new(Self {
            base: main.base.clone(),
            step,
            index: index.clone(),
            left: MainCard::new(
                main.base.clone(),
                step.clone(),
                index.clone(),
                Side::Left,
                pair.0.clone(),
                pair.1.clone()
            ),
            right: MainCard::new(
                main.base.clone(),
                step.clone(),
                index.clone(),
                Side::Right,
                pair.1.clone(),
                pair.0.clone()
            )
        })
    }
}

