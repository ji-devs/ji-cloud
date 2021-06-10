use components::module::edit::prelude::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;


impl DomRenderable for Footer {
    fn render(state: Rc<Footer>) -> Dom {
        html!("empty-fragment")
    }
}
