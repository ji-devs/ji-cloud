use futures_signals::signal::{Mutable, Signal, SignalExt};
use super::{
    animation::{Animation, Transform},
    super::state::*
};
use dominator::clone;
use dominator_helpers::signals::{DefaultSignal, OptionSignal};
use std::cell::RefCell;
use web_sys::HtmlElement;

use shared::domain::jig::{
    JigId, 
    Jig, 
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Audio,
            Image,
            memory::{CardPair as RawCardPair, Card as RawCard}
        }
    }
};
use wasm_bindgen::prelude::*;
use utils::prelude::*;

#[derive(Clone)]
pub struct CardState {
    pub media: Media,
    pub id: usize,
    pub other_id: usize,
    pub side: Side,
    pub found_index: Mutable<Option<usize>>,
    pub animation: Mutable<Option<Animation>>, 
    pub main_elem: RefCell<Option<HtmlElement>>,
}

impl CardState {
    pub fn new(media:Media, id: usize, other_id:usize, side:Side) -> Self {
        Self { 
            media,
            id,
            other_id,
            side,
            found_index: Mutable::new(None),
            animation: Mutable::new(None),
            main_elem: RefCell::new(None),
        }
    }

    //this is tied to animaton instead of found_index
    //so that the visual transition happens only when
    //the proper transform is being set
    pub fn is_found(&self) -> impl Signal<Item = bool> {
        self.animation.signal_ref(|x| x.is_some())
    }

    pub fn is_flipped(&self, base: &Base) -> impl Signal<Item = bool> {
        let self_id = self.id.clone();

        base.flip_state
            .signal_ref(clone!(self_id => move |flip_state| {
                match flip_state {
                    FlipState::None => false,
                    FlipState::One(id) => id == &self_id,
                    FlipState::Two(id_1, id_2) => id_1 == &self_id || id_2 == &self_id
                }
            }))
    }

    pub fn transform_signal(&self) -> impl Signal<Item = Option<Transform>> {
        self.animation.signal_ref(|anim| {
            OptionSignal::new( 
                anim.as_ref().map(|anim| anim.transform_signal())
            )
        })
        .flatten()
    }

    //After found animation has completed
    pub fn ended_signal(&self) -> impl Signal<Item = bool> {
        self.animation.signal_ref(|anim| {
            DefaultSignal::new(
                false,
                anim.as_ref().map(|anim| anim.ended_signal())
            )
        })
        .flatten()
    }
}


#[derive(Debug, Clone)]
pub enum Media{
    Text(String),
    Image(Image),
}

impl From<&RawCard> for Media {
    fn from(card:&RawCard) -> Self {
        match card {
            RawCard::Text(x) => Self::Text(x.to_string()),
            RawCard::Image(image) => Self::Image(image.as_ref().unwrap_ji().clone())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}


impl From<Side> for app_memory_common::lookup::Side {
    fn from(side: Side) -> Self {
        match side {
            Side::Left => app_memory_common::lookup::Side::Left,
            Side::Right => app_memory_common::lookup::Side::Right,
        }
    }
}

impl Side {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
