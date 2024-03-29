use super::state::*;
use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use dominator::{html, Dom};
use std::rc::Rc;

impl<RawData: RawDataExt, E: ExtraExt> DomRenderable for Footer<RawData, E> {
    fn render(_state: Rc<Footer<RawData, E>>) -> Dom {
        html!("empty-fragment")
    }
}
