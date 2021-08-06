use super::state::*;
use components::module::_common::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;

impl DomRenderable for Footer {
    fn render(_state: Rc<Footer>) -> Dom {
        html!("empty-fragment")
    }
}
