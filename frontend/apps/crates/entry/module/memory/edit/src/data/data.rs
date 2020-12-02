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
use super::raw;
use itertools::Itertools;
use std::fmt::Write;
use serde::Deserialize;

pub struct State {
    pub jig_id: String,
    pub module_id: String,
    pub game_mode: Mutable<Option<GameMode>>,
    pub pairs: MutableVec<(Card, Card)>,
    pub step: Mutable<Step>,
    pub theme_id: Mutable<String>,
    pub first_text: RefCell<bool>,
    pub content_mode: Mutable<ContentMode>
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
            raw::Card::Text(x) => (CardMode::Text,Some(x)),
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
            CardMode::Text => raw::Card::new_text(card.data.get_cloned().unwrap_or("".to_string())) ,
            CardMode::Image => raw::Card::new_image(card.data.get_cloned()), 
            CardMode::Audio => raw::Card::new_audio(card.data.get_cloned()), 
        }

    }
}

#[derive(Debug, Clone)]
pub enum CardMode {
    Text,
    Image,
    Audio
}

impl State {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            game_mode: Mutable::new(None),
            pairs: MutableVec::new(),
            step: Mutable::new(debug::settings().step.unwrap_or(Step::One)),
            theme_id: Mutable::new(crate::config::get_themes_cloned()[0].id.clone()),
            first_text: RefCell::new(true),
            content_mode: Mutable::new(debug::settings().content_mode)
        }
    }

    pub fn set_from_raw(&self, raw_data:Option<raw::GameData>) {
        self.game_mode.set(raw_data.as_ref().map(|data| data.mode.clone().into()));

        if let Some(raw_data) = raw_data {
            let pairs:Vec<(Card, Card)> = raw_data
                .pairs
                .iter()
                .map(|(left, right)| {
                    (left.clone().into(), right.clone().into())
                })
                .collect();
            self.pairs.lock_mut().replace_cloned(pairs);

            self.theme_id.set(raw_data.theme_id);
        } else {
            self.pairs.lock_mut().clear();
        }
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
            theme_id: self.theme_id.get_cloned()
        }
    }

    pub fn change_mode(&self, mode: GameMode) {
        self.game_mode.set(Some(mode));
        self.pairs.lock_mut().clear();
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


#[derive(Deserialize, Clone)]
pub struct Theme {
    pub id: String,
    pub content: String,
    pub label: String
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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}
