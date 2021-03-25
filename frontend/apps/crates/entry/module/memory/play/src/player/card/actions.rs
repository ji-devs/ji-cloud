use crate::data::state::{State as AppState, FlipState};
use super::{
    state::State as CardState,
    animation::Animation
};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use web_sys::HtmlElement;
use wasm_bindgen::prelude::*;

pub fn card_click(state: Rc<AppState>, id: usize) -> Option<(usize, usize)> {
    let flip_state = &mut *state.flip_state.lock_mut();

    match flip_state {
        FlipState::None => {
            *flip_state = FlipState::One(id);
            None
        }
        FlipState::One(other) => {
            let other = *other;
            if other != id {
                *flip_state = FlipState::Two(id, other);
                Some((id, other))
            } else {
                None
            }
        },
        _ => None
    }
}

pub fn evaluate(state: Rc<AppState>, id_1: usize, id_2: usize) {
    spawn_local(async move {
        if state.pair_lookup[id_1] == id_2 {
            let mut found_pairs = state.found_pairs.borrow_mut();
            let found_pairs_index = found_pairs.len();
            found_pairs.push((id_1, id_2));
            if let Some(card) = state.cards.iter().find(|c| c.id == id_1) {
                card.found_index.set(Some(found_pairs_index));
            }
            if let Some(card) = state.cards.iter().find(|c| c.id == id_2) {
                card.found_index.set(Some(found_pairs_index));
            }
        } else {
            TimeoutFuture::new(2_000).await;
        }
        state.flip_state.set(FlipState::None);
    })
}

pub fn start_animation(state: &AppState, card: Rc<CardState>, found_index: usize) {
    card.animation.set(Some(Animation::new(state, card.main_elem.borrow().as_ref().unwrap_throw(), found_index, card.side)));
}
