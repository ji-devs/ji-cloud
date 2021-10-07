use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use std::rc::Rc;
use futures_signals::{map_ref, signal::{Signal, SignalExt, Mutable}};
use shared::domain::jig::module::body::_groups::cards::Step;

pub struct Header<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
}

impl<RawData: RawDataExt, E: ExtraExt> Header<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>) -> Self {
        Self { base }
    }

    pub fn show_add_pair_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let step = self.base.step.signal_cloned(),
            let is_empty = self.base.is_empty_signal()
            => {
                match step {
                    Step::One => !is_empty,
                    _ => false
                }
            }
        }
    }
}

impl<RawData: RawDataExt, E: ExtraExt> HeaderExt for Header<RawData, E> {}
