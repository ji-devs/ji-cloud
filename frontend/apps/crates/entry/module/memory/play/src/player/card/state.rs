use futures_signals::signal::{Mutable, Signal, SignalExt};

#[derive(Clone, Debug)]
pub struct State {
    pub media: Media,
    pub id: usize,
    pub other_id: usize,
    pub side: Side,
    pub is_found: Mutable<bool>,
    pub flip: Mutable<bool>
}

impl State {
    pub fn new(media:&Media, id: usize, other_id:usize, side:Side) -> Self {
        Self { 
            media: media.clone(),
            id,
            other_id,
            side,
            is_found: Mutable::new(false),
            flip: Mutable::new(false)
        }
    }
}

pub type Media = crate::data::raw::Card;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}
