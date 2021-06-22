use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{signal::{ReadOnlyMutable, SignalExt}, signal_vec::SignalVecExt};
use crate::module::_groups::cards::edit::{
    state::*,
    main::state::Main
};
use super::card::state::*;

use shared::domain::jig::module::body::_groups::cards::Step;
pub struct MainPair <RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub step: Step, 
    pub index: ReadOnlyMutable<Option<usize>>,
    pub left: Rc<MainCard<RawData, E>>,
    pub right: Rc<MainCard<RawData, E>>
}

impl <RawData: RawDataExt, E: ExtraExt> MainPair<RawData, E> {
    pub fn new(
        base: Rc<CardsBase<RawData, E>>, 
        step: Step, 
        index: ReadOnlyMutable<Option<usize>>,
        pair: (Card, Card)
    ) -> Rc<Self> {
        Rc::new(Self {
            base: base.clone(),
            step,
            index: index.clone(),
            left: MainCard::new(
                base.clone(),
                step.clone(),
                index.clone(),
                Side::Left,
                pair.0.clone(),
                pair.1.clone()
            ),
            right: MainCard::new(
                base.clone(),
                step.clone(),
                index.clone(),
                Side::Right,
                pair.1.clone(),
                pair.0.clone()
            )
        })
    }
}

