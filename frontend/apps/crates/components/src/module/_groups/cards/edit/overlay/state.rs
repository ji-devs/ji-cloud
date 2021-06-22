use utils::unwrap::UnwrapJiExt;
use std::rc::Rc;
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use crate::module::{
    edit::prelude::*,
    _groups::cards::edit::state::*,

};


pub struct Overlay <RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
}

impl <RawData: RawDataExt, E: ExtraExt> Overlay<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>) -> Self {
        Self {
            base 
        }
    }
}

impl <RawData: RawDataExt, E: ExtraExt> OverlayExt for Overlay<RawData, E> {
}
