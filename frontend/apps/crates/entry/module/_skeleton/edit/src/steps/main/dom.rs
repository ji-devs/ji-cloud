use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;


impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment")
    }
}
