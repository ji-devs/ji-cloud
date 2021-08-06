use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use std::rc::Rc;

pub struct Header<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
}

impl<RawData: RawDataExt, E: ExtraExt> Header<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>) -> Self {
        Self { base }
    }
}

impl<RawData: RawDataExt, E: ExtraExt> HeaderExt for Header<RawData, E> {}
