use std::rc::Rc;
use utils::prelude::*;
use crate::base::{
    state::*,
    card::{
        state::*,
        animation::*
    }
};

pub fn start_animation(state: &Base, card: Rc<CardState>, found_index: usize) {
    card.animation.set(Some(Animation::new(state, card.main_elem.borrow().as_ref().unwrap_ji(), found_index, card.side)));
}
