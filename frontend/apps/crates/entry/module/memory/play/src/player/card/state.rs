use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::data::state::{FlipState, State as AppState};
use dominator::clone;

#[derive(Clone, Debug)]
pub struct State {
    pub media: Media,
    pub id: usize,
    pub other_id: usize,
    pub side: Side,
    pub found_index: Mutable<Option<usize>>,
}

impl State {
    pub fn new(media:&Media, id: usize, other_id:usize, side:Side) -> Self {
        Self { 
            media: media.clone(),
            id,
            other_id,
            side,
            found_index: Mutable::new(None),
        }
    }

    pub fn is_found(&self) -> impl Signal<Item = bool> {
        self.found_index.signal_cloned().map(|x| x.is_some())
    }

    pub fn is_flipped(&self, app_state: &AppState) -> impl Signal<Item = bool> {
        let self_id = self.id.clone();

        app_state.flip_state
            .signal_ref(clone!(self_id => move |flip_state| {
                match flip_state {
                    FlipState::None => false,
                    FlipState::One(id) => id == &self_id,
                    FlipState::Two(id_1, id_2) => id_1 == &self_id || id_2 == &self_id
                }
            }))
    }
}

pub type Media = crate::data::raw::Card;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}
