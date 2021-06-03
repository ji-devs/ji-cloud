use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;


impl DomRenderable for Footer {
    fn render(state: Rc<Footer>) -> Dom {
        html!("empty-fragment")
    }
}
