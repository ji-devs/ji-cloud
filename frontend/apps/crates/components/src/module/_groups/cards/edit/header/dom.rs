use super::state::*;
use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use dominator::{html, Dom};
use std::rc::Rc;

impl<RawData: RawDataExt, E: ExtraExt> DomRenderable for Header<RawData, E> {
    fn render(_state: Rc<Header<RawData, E>>) -> Dom {
        html!("empty-fragment", {
            .child(html!("h1", {.text("HELLO")}))
        })
    }
}
