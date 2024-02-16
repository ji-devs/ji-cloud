use crate::base::{
    card::{animation::*, state::*},
    state::*,
};
use std::rc::Rc;
use utils::prelude::*;

pub fn start_animation(state: &Base, card: Rc<CardState>) {
    state.is_animating.set(state.is_animating.get() + 1);
    card.animation.set(Some(Animation::new(
        state,
        card.main_elem.borrow().as_ref().unwrap_ji(),
        card.side,
    )));
}
