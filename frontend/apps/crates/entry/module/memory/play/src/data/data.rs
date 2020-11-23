use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
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

pub struct GameState {
    pub jig_id: String,
    pub module_id: String,
    //outer option is for "loading", inner option is for "no module chosen"
    pub mode: Mutable<Option<Option<GameMode>>>, 
    pub state: Rc<RefCell<Option<BaseGameState>>>,
}


impl GameState {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            mode: Mutable::new(None),
            state: Rc::new(RefCell::new(None))
        }
    }

    pub fn set_from_loaded(&self, raw_game_state:raw::GameState) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let (mode, state) = match raw_game_state {
            raw::GameState::Duplicate(raw_state) => {
                let mode = GameMode::Duplicate;
                (
                    Some(mode),
                    Some(BaseGameState::from_raw(mode, raw_state, self.jig_id.clone(), self.module_id.clone()))
                )
            },
            raw::GameState::WordsAndImages(raw_state) => {
                let mode = GameMode::WordsAndImages;
                (
                    Some(mode),
                    Some(BaseGameState::from_raw(mode, raw_state, self.jig_id.clone(), self.module_id.clone()))
                )
            },
            raw::GameState::None => (None, None),
            _ => unimplemented!("no way to load {:?}", raw_game_state) 
        };

        //Note that this will *not* trigger re-renders of the inner mode pages
        //Using set_from_loaded for runtime changes is therefore very inefficient!
        //It's only meant for first-time loading
        *self.state.borrow_mut() = state;
        //wrapped in a Some because None here means "loading"
        //this *will* trigger re-renders of everything from the top-level
        //an inner none means "loaded but no mode"
        self.mode.set(Some(mode));
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Duplicate,
    WordsAndImages,
}

#[derive(Clone, Debug)]
pub struct Card {
    pub media: Media,
    pub id: usize,
    pub other_id: usize,
    pub found: Mutable<bool>
}

impl Card {
    pub fn new(card:&raw::Card, id: usize, other_id:usize) -> Self {
        Self {
            media: match card {
                raw::Card::Text(text) => Media::Text(text.to_string()),
                raw::Card::Image(text) => Media::Image(text.to_string()),
                raw::Card::Audio(text) => Media::Audio(text.to_string()),
            },
            id,
            other_id,
            found: Mutable::new(false),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Media {
    Text(String),
    Image(String),
    Audio(String),
}

#[derive(Debug)]
pub struct BaseGameState {
    pub jig_id: String,
    pub module_id: String,
    pub pair_lookup: Vec<usize>,
    pub original_pairs: Vec<(raw::Card, raw::Card)>,
    pub game_cards: MutableVec<Card>,
    pub theme_id: String,
    pub flip_state: Mutable<FlipState>,
}

#[derive(Debug, Clone)]
pub enum FlipState {
    None,
    One(usize),
    Two((usize, usize)),
}
#[derive(Debug, PartialEq)]
enum Side {
    Right,
    Left
}

fn make_game_cards(pairs:&[(raw::Card, raw::Card)]) -> Vec<Card> {
    let n_cards = pairs.len() * 2;
    let mut cards:Vec<Card> = Vec::with_capacity(n_cards);
    let mut index:usize = 0;

    for (card_1, card_2) in pairs.iter() {
        let id_1 = index; 
        let id_2 = index + 1;
        index = id_2 + 1;

        cards.push(Card::new(card_1, id_1, id_2));
        cards.push(Card::new(card_2, id_2, id_1));
    }

    cards
}

impl BaseGameState {
    pub fn from_raw(mode: GameMode, raw_game_state: raw::BaseGameState, jig_id: String, module_id: String) -> Self {
        let n_cards = raw_game_state.pairs.len() * 2;
        let mut pair_lookup:Vec<usize> = vec![0;n_cards]; 
        let mut cards = make_game_cards(&raw_game_state.pairs);

        for card in cards.iter() {
            pair_lookup[card.id] = card.other_id;
        }

        let mut rng = thread_rng();

        if debug::settings().shuffle {
            cards.shuffle(&mut rng);
        }

        let state = Self {
            jig_id,
            module_id,
            pair_lookup,
            original_pairs: raw_game_state.pairs,
            game_cards: MutableVec::new_with_values(cards),
            theme_id: raw_game_state.theme_id,
            flip_state: Mutable::new(FlipState::None), 
        };

        state
    }

    pub async fn evaluate(&self, id_1: usize, id_2: usize) {

        if self.pair_lookup[id_1] == id_2 {
            let game_cards = self.game_cards.lock_ref();
            if let Some(card) = game_cards.iter().find(|c| c.id == id_1) {
                card.found.set(true);
            }
            if let Some(card) = game_cards.iter().find(|c| c.id == id_2) {
                card.found.set(true);
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

