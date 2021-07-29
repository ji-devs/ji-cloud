use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use crate::module::{
    _groups::cards::edit::state::*,
    _common::edit::prelude::*,
};

impl <RawData: RawDataExt, E: ExtraExt> DomRenderable for Footer<RawData, E> {
    fn render(state: Rc<Footer<RawData, E>>) -> Dom {
        html!("empty-fragment")
    }
}
