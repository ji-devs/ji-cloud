use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use components::image::data::*;
use shared::media::{image_id_to_key, MediaLibraryKind, MediaVariant};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::debug;
use super::raw;
use itertools::Itertools;
use std::fmt::Write;
use rand::prelude::*;
use gloo_timers::future::TimeoutFuture;
use web_sys::HtmlElement;

pub struct State {
    pub jig_id: String,
    pub module_id: String,
    pub mode: GameMode,
    pub pair_lookup: Vec<usize>,
    pub original_pairs: Vec<(raw::Card, raw::Card)>,
    pub game_cards: MutableVec<Card>,
    pub theme_id: String,
    pub flip_state: Mutable<FlipState>,
    pub found_pairs: RefCell<Vec<(usize, usize)>>, 
}

impl State {
    pub fn new(jig_id:String, module_id: String, raw_data:raw::GameData) -> Rc<Self> {

        let mode:GameMode = raw_data.mode.into();

        let n_cards = raw_data.pairs.len() * 2;
        let mut pair_lookup:Vec<usize> = vec![0;n_cards]; 
        let mut cards = make_game_cards(&raw_data.pairs);

        for card in cards.iter() {
            pair_lookup[card.id] = card.other_id;
        }

        let mut rng = thread_rng();

        if debug::settings().shuffle {
            cards.shuffle(&mut rng);
        }

        Rc::new(Self {
            jig_id,
            module_id,
            mode,
            pair_lookup,
            original_pairs: raw_data.pairs,
            game_cards: MutableVec::new_with_values(cards),
            theme_id: raw_data.theme_id,
            flip_state: Mutable::new(FlipState::None), 
            found_pairs: RefCell::new(Vec::new()),
        })

    }

    pub async fn evaluate(&self, id_1: usize, id_2: usize) {

        if self.pair_lookup[id_1] == id_2 {
            let game_cards = self.game_cards.lock_ref();
            let mut found_pairs = self.found_pairs.borrow_mut();
            let found_pairs_index = found_pairs.len();
            found_pairs.push((id_1, id_2));
            if let Some(card) = game_cards.iter().find(|c| c.id == id_1) {
                card.found.set(Some(found_pairs_index));
            }
            if let Some(card) = game_cards.iter().find(|c| c.id == id_2) {
                card.found.set(Some(found_pairs_index));
            }
        } else {
            TimeoutFuture::new(2_000).await;
        }
        self.flip_state.set(FlipState::None);
    }
    pub fn grid_number(&self) -> usize { 
        let n_cards = self.game_cards.lock_ref().len();
        match n_cards {
            8 => 1,
            10 => 2,
            12 => 1,
            14 => 5,
            16 => 1,
            18 => 6,
            20 => 2,
            22 => 7,
            24 => 3,
            26 => 8,
            28 => 4,
            _ => panic!("no known grid number for {} cards!", n_cards)
        }
    }

    pub fn card_click(&self, id: usize) {
        let flip_state = &mut *self.flip_state.lock_mut();

        match flip_state {
            FlipState::None => *flip_state = FlipState::One(id),
            FlipState::One(other) => {
                if *other != id {
                    *flip_state = FlipState::Two((id, *other))
                }
            },
            _ => {}
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

pub type FoundIndex = usize;

#[derive(Clone, Debug)]
pub struct Card {
    pub media: Media,
    pub id: usize,
    pub other_id: usize,
    pub side: Side,
    pub found: Mutable<Option<FoundIndex>>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

impl Card {
    pub fn new(card:&raw::Card, id: usize, other_id:usize, side:Side) -> Self {
        Self {
            media: match card {
                raw::Card::Text(text) => Media::Text(text.to_string()),
                raw::Card::Image(id) => Media::Image(id.as_ref().map(|id| {
                    SimpleImage::from((id.to_string(), MediaLibraryKind::Global))
                })),
                raw::Card::Audio(src) => Media::Audio(src.clone()),
            },
            id,
            other_id,
            found: Mutable::new(None),
            side,
        }
    }
}

type Id = String;

#[derive(Clone, Debug)]
pub enum Media {
    Text(String),
    Image(Option<SimpleImage>),
    Audio(Option<Id>),
}

#[derive(Debug, Clone)]
pub enum FlipState {
    None,
    One(usize),
    Two((usize, usize)),
}

fn make_game_cards(pairs:&[(raw::Card, raw::Card)]) -> Vec<Card> {
    let n_cards = pairs.len() * 2;
    let mut cards:Vec<Card> = Vec::with_capacity(n_cards);
    let mut index:usize = 0;

    for (card_1, card_2) in pairs.iter() {
        let id_1 = index; 
        let id_2 = index + 1;
        index = id_2 + 1;

        cards.push(Card::new(card_1, id_1, id_2, Side::Left));
        cards.push(Card::new(card_2, id_2, id_1, Side::Right));
    }

    cards
}

