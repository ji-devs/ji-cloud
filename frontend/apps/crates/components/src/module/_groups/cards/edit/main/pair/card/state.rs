use dominator::{html, Dom, clone};
use std::{rc::Rc, cell::RefCell};
use web_sys::HtmlElement;
use futures_signals::{signal::{Mutable, ReadOnlyMutable, SignalExt}, signal_vec::SignalVecExt};
use crate::module::_groups::cards::{lookup, edit::state::*};
use super::callbacks::*;
use shared::domain::jig::module::body::_groups::cards::Step;

pub struct MainCard <RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub step: Step, 
    pub index: ReadOnlyMutable<Option<usize>>,
    pub side: Side,
    pub card: Card,
    pub other: Card,
    pub input_ref:Rc<RefCell<Option<HtmlElement>>>,
    pub editing_active:Mutable<bool>,
    pub is_image: bool,
    pub callbacks: CardCallbacks
}

impl <RawData: RawDataExt, E: ExtraExt> MainCard <RawData, E> {
    pub fn new(
        base: Rc<CardsBase<RawData, E>>,
        step: Step,
        index: ReadOnlyMutable<Option<usize>>,
        side: Side,
        card: Card,
        other: Card,
    ) -> Rc<Self> {
        let is_image = match card {
            Card::Image(_) => true,
            _ => false
        };

        let callbacks = {
            if is_image {
                CardCallbacks::new(Some(clone!(base => move || {
                })))
            } else {
                CardCallbacks::new(None::<fn()>)
            }
        };
        Rc::new(Self {
            base,
            step,
            index,
            side,
            card,
            other,
            input_ref: Rc::new(RefCell::new(None)),
            editing_active: Mutable::new(false),
            is_image,
            callbacks
        })
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

impl From<Side> for lookup::Side {
    fn from(side: Side) -> Self {
        match side {
            Side::Left => lookup::Side::Left,
            Side::Right => lookup::Side::Right,
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

    pub const fn slot_name(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
//pub fn render(state:Rc<State>, mode: Mode, step: Step, index: ReadOnlyMutable<Option<usize>>, side:Side, card: Card, other: Card) -> Dom {
