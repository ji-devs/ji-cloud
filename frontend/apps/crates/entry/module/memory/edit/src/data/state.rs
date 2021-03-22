use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods, with_node};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::{raw, history::History};
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;
use crate::index::state::LocalState as IndexState;
use std::collections::HashSet;
use components::module::history::state::HistoryState;

pub struct State {
    pub index: Rc<IndexState>,
    pub game_mode: Mutable<Option<GameMode>>,
    pub pairs: MutableVec<(Card, Card)>,
    pub step: Mutable<Step>,
    pub steps_completed: Mutable<HashSet<Step>>,
    pub theme: Mutable<String>,
    pub first_text: RefCell<bool>,
    pub content_mode: Mutable<ContentMode>,
    pub history: Rc<HistoryState<History>>,

    pub is_empty: Mutable<bool>, //to avoid re-rendering things that only depend on this state change
}


impl State {
    pub fn new(index: Rc<IndexState>, raw_data:Option<raw::GameData>) -> Self {

        let game_mode:Option<GameMode> = raw_data.as_ref().map(|data| data.mode.clone().into());

        let (pairs, theme) = {
            if let Some(raw_data) = &raw_data {
                let pairs:Vec<(Card, Card)> = raw_data.pairs
                    .iter()
                    .map(|(left, right)| {
                        (left.clone().into(), right.clone().into())
                    })
                    .collect();

                (pairs, raw_data.theme.clone())
            } else {
                (
                    Vec::new(),
                    crate::config::get_themes_cloned()[0].clone()
                )
            }
        };

        let is_empty = pairs.is_empty();

        let step = index.step.clone();
        Self {
            index,
            game_mode: Mutable::new(game_mode),
            pairs: MutableVec::new_with_values(pairs),
            step,
            steps_completed: Mutable::new(HashSet::new()),
            theme: Mutable::new(theme),
            first_text: RefCell::new(true),
            content_mode: Mutable::new(debug::settings().content_mode),
            history: Rc::new(HistoryState::new(History::new(raw_data))),
            is_empty: Mutable::new(is_empty)
        }
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> {
        self.is_empty.signal()
    }
    pub fn pairs_len_signal(&self) -> impl Signal<Item = usize> {
        self.pairs.signal_vec_cloned().len()
    }

    pub fn has_content_signal(&self) -> impl Signal<Item = bool> {
        self.pairs_len_signal()
            .map(|len| len > 0)
    }


    pub fn to_raw(&self) -> raw::GameData {
        let game_mode = self.game_mode.get().unwrap_throw();

        let mode:raw::Mode = game_mode.into();

        let pairs:Vec<(raw::Card, raw::Card)> = self.pairs.lock_ref()
            .iter()
            .map(|(left, right)| {
                (left.clone().into(), right.clone().into())
            })
            .collect();

        raw::GameData {
            mode,
            pairs,
            theme: self.theme.get_cloned()
        }
    }


    pub fn cards_edit_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let step = self.step.signal(),
            let content_mode = self.content_mode.signal()
            => {
                let step = *step;
                let content_mode = *content_mode;

                if step == Step::One && content_mode != ContentMode::TextInit {
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn cards_hide_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let step = self.step.signal(),
            let content_mode = self.content_mode.signal()
            => {
                let step = *step;
                let content_mode = *content_mode;

                if step == Step::One && content_mode == ContentMode::TextInit {
                    true
                } else {
                    false
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Card {
    pub mode: Mutable<CardMode>,
    pub data: Mutable<Option<String>>,
}

impl Card {
    pub fn new(mode:CardMode) -> Self {
        Self {
            mode: Mutable::new(mode),
            data: Mutable::new(None),
        }
    }
    pub fn new_with_data(mode:CardMode, data:String) -> Self {
        Self {
            mode: Mutable::new(mode),
            data: Mutable::new(Some(data)),
        }
    }
}

impl From<raw::Card> for Card {
    fn from(raw_card:raw::Card) -> Self {

        let (mode, data) = match raw_card {
            raw::Card::Text(x) => (CardMode::Text,x),
            raw::Card::Image(x) => (CardMode::Image,x),
            raw::Card::Audio(x) => (CardMode::Audio,x)
        };

        Self {
           mode: Mutable::new(mode),
           data: Mutable::new(data)
        }
    }
}
impl From<Card> for raw::Card {
    fn from(card:Card) -> raw::Card {
        
        match card.mode.get_cloned() {
            CardMode::Text => raw::Card::Text(card.data.get_cloned()) ,
            CardMode::Image => raw::Card::Image(card.data.get_cloned()), 
            CardMode::Audio => raw::Card::Audio(card.data.get_cloned()), 
        }

    }
}

#[derive(Debug, Clone)]
pub enum CardMode {
    Text,
    Image,
    Audio
}


#[derive(Clone, Copy, PartialEq)]
pub enum GameMode {
    Duplicate,
    WordsAndImages
}

impl From<raw::Mode> for GameMode {
    fn from(raw_mode:raw::Mode) -> Self {
        match raw_mode {
            raw::Mode::Duplicate => Self::Duplicate,
            raw::Mode::WordsAndImages => Self::WordsAndImages,
        }
    }
}
impl From<GameMode> for raw::Mode {
    fn from(game_mode:GameMode) -> raw::Mode {
        match game_mode {
            GameMode::Duplicate => raw::Mode::Duplicate,
            GameMode::WordsAndImages => raw::Mode::WordsAndImages,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ContentMode {
    TextInit,
    TextDone,
    Images
}


#[derive(Clone, Copy, PartialEq)]
pub enum PairType {
    TextText,
    TextImage
}

impl PairType {
    pub fn left_is_text(&self) -> bool {
        true
    }
    pub fn left_is_image(&self) -> bool {
        false 
    }
    pub fn right_is_text(&self) -> bool {
        match self {
            Self::TextText => true,
            Self::TextImage => false,
        }
    }
    pub fn right_is_image(&self) -> bool {
        match self {
            Self::TextText => false,
            Self::TextImage => true,
        }
    }

}

impl From<(&CardMode, &CardMode)> for PairType {
    fn from((left, right):(&CardMode, &CardMode)) -> PairType {
        match (left, right) {
            (CardMode::Text, CardMode::Text) => PairType::TextText,
            (CardMode::Text, CardMode::Image) => PairType::TextImage,
            _ => unimplemented!("unknown card pair type!")
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::One => crate::strings::steps_nav::STR_CONTENT,
            Step::Two => crate::strings::steps_nav::STR_DESIGN,
            Step::Three => crate::strings::steps_nav::STR_SETTINGS,
            Step::Four => crate::strings::steps_nav::STR_PREVIEW,
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Step::One => 1, 
            Step::Two => 2, 
            Step::Three => 3, 
            Step::Four => 4 
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn slot_name(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}
