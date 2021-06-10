use components::module::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;


impl DomRenderable for Header {
    fn render(state: Rc<Header>) -> Dom {
        html!("empty-fragment")
    }
}
