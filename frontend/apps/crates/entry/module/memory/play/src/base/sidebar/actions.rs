use crate::base::{
    card::{animation::*, state::*},
    state::*,
};
use std::rc::Rc;
use utils::prelude::*;

pub fn start_animation(state: &Base, card: Rc<CardState>, found_index: usize) {
    card.animation.set(Some(Animation::new(
        state,
        card.main_elem.borrow().as_ref().unwrap_ji(),
        found_index,
        card.side,
    )));
}
