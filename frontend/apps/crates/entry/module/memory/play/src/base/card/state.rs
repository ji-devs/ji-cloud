use futures_signals::signal::{Mutable, Signal, SignalExt};
use super::{
    animation::{Animation, AnimationState},
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
            _groups::cards::{CardPair as RawCardPair, Card}
        }
    }
};
use wasm_bindgen::prelude::*;
use utils::prelude::*;
use components::module::_groups::cards::lookup::Side;

#[derive(Clone)]
pub struct CardState {
    pub card: Card,
    pub id: usize,
    pub other_id: usize,
    pub side: Side,
    pub found_index: Mutable<Option<usize>>,
    pub animation: Mutable<Option<Animation>>, 
    pub main_elem: RefCell<Option<HtmlElement>>,
}

impl CardState {
    pub fn new(card:Card, id: usize, other_id:usize, side:Side) -> Self {
        Self { 
            card,
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

    pub fn animation_state_signal(&self) -> impl Signal<Item = Option<AnimationState>> {
        self.animation.signal_ref(|anim| {
            OptionSignal::new( 
                anim.as_ref().map(|anim| anim.state_signal())
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

