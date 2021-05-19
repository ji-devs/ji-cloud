use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;

impl DomRenderable for Sidebar {
    fn render(state: Rc<Sidebar>) -> Dom {
        html!("empty-fragment")
    }
}
