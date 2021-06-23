use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::state::*;
use super::state::*;
use components::module::_groups::cards::edit::main::dom::render_main_cards;
use shared::domain::jig::module::body::_groups::cards::Step;

pub fn render(state: Rc<MainSettings>) -> Dom {
    render_main_cards(state.base.clone(), Step::Three)
}

