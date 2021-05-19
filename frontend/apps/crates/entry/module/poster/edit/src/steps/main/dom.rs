use components::module::edit::*;
use dominator::{html, Dom};
use std::rc::Rc;
use super::state::*;
use components::{backgrounds, stickers};

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .property("slot", "main")
            .child(backgrounds::dom::render(state.base.backgrounds.clone(), None))
            .child(stickers::dom::render(state.base.stickers.clone(), None))
        })
    }
}
