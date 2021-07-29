use std::rc::Rc;
use crate::module::{
    _common::edit::prelude::*,
    _groups::cards::edit::state::*,
};


pub struct Footer <RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
}

impl <RawData: RawDataExt, E: ExtraExt> Footer <RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>) -> Self {
        Self {
            base,
        }
    }
}

impl <RawData: RawDataExt, E: ExtraExt> FooterExt for Footer<RawData, E> {
}
