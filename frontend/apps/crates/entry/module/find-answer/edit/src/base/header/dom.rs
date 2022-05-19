use super::state::*;
use components::module::_common::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;

impl DomRenderable for Header {
    fn render(_state: Rc<Header>) -> Dom {
        html!("empty-fragment")
    }
}
