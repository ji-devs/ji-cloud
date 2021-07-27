use components::module::_common::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;

impl DomRenderable for Overlay {
    fn render(_state: Rc<Overlay>) -> Dom {
        html!("empty-fragment", {
        })
    }
}
