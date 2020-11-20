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
use super::raw::*;
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
    pub mode_state: Rc<RefCell<Option<ModeState>>>,
}


impl GameState {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            mode: Mutable::new(None),
            mode_state: Rc::new(RefCell::new(None))
        }
    }
    pub fn set_from_loaded(&self, raw:GameStateRaw) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let (mode, mode_state) = match raw {
            GameStateRaw::Duplicate(raw_state) => {
                (
                    Some(GameMode::Duplicate),
                    Some(ModeState::Duplicate(Rc::new(DuplicateState::from_raw(self.jig_id.clone(), self.module_id.clone(), raw_state))))
                )
            },
            _ => (None, None)
        };

        //Note that this will *not* trigger re-renders of the inner mode pages
        //Using set_from_loaded for runtime changes is therefore very inefficient!
        //It's only meant for first-time loading
        *self.mode_state.borrow_mut() = mode_state;
        //wrapped in a Some because None here means "loading"
        //this *will* trigger re-renders of everything from the top-level
        //an inner none means "loaded but no mode"
        self.mode.set(Some(mode));
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    Duplicate
}

#[derive(Debug)]
pub enum ModeState {
    Duplicate(Rc<DuplicateState>)
}

#[derive(Clone, Debug)]
pub struct Card {
    pub text: String,
    pub id: usize, 
}

#[derive(Clone, Debug)]
pub struct GameCard {
    pub text: String,
    pub id: usize, 
    pub found: Mutable<bool>
}

#[derive(Debug, Clone)]
pub struct FlipCard {
    pub id: usize,
    pub element: HtmlElement
}

#[derive(Clone, Debug)]
pub struct HoverCard {
    pub text: String,
    pub id: usize,
    pub origin_x: f64,
    pub origin_y: f64,
    pub dest_x: f64,
    pub dest_y: f64,
}

#[derive(Debug, Clone)]
pub struct FoundCard {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub text: String
}

impl FoundCard {
    pub fn new(card:&HoverCard) -> Self {
        Self {
            id: card.id,
            x: card.dest_x,
            y: card.dest_y,
            text: card.text.to_string()
        }
    }
}


impl Card {
    pub fn new(card:CardRaw, id: usize) -> Self {
        Self {
            text: card.text,
            id,
        }
    }
}

#[derive(Debug)]
pub struct DuplicateState {
    pub jig_id: String,
    pub module_id: String,
    pub pair_lookup: Vec<usize>,
    pub all_cards: Vec<Card>,
    pub game_cards: MutableVec<GameCard>,
    pub found_cards: MutableVec<FoundCard>,
    pub hover_cards: MutableVec<HoverCard>,
    pub theme_id: String,
    pub flip_state: Mutable<FlipState>,
}

#[derive(Debug, Clone)]
pub enum FlipState {
    None,
    One(FlipCard),
    Two((FlipCard, FlipCard)),
}
#[derive(Debug, PartialEq)]
enum Side {
    Right,
    Left
}

impl DuplicateState {
    pub fn make_hover_card(&self, flip_card:FlipCard, side: Side) -> HoverCard {
        let card = &self.all_cards[flip_card.id];

        let (origin_x, origin_y) = utils::resize::ModuleBounds::get_element_pos_rem(&flip_card.element.unchecked_into());
      
        let mut len = self.found_cards.lock_ref().len();
        len += self.hover_cards.lock_ref().len();
        let dest_y = (len as f64) * 10.0;
        let dest_x = match side {
            Side::Right => 10.0,
            Side::Left => 0.0
        };

        HoverCard {
            text: card.text.clone(),
            id: card.id,
            origin_x,
            origin_y,
            dest_x,
            dest_y,
        }
    }

    pub fn from_raw(jig_id: String, module_id: String, raw:DuplicateStateRaw) -> Self {
        let n_cards = raw.cards.len() * 2;
        let mut all_cards:Vec<Card> = Vec::with_capacity(n_cards);
        let mut pair_lookup:Vec<usize> = vec![0;n_cards]; 
        let mut index:usize = 0;

        for raw_card in raw.cards.into_iter() {
            let id_1 = index; 
            let id_2 = index + 1;
            index = id_2 + 1;

            all_cards.push(Card::new(raw_card.clone(), id_1));
            all_cards.push(Card::new(raw_card, id_2));
            pair_lookup[id_1] = id_2;
            pair_lookup[id_2] = id_1;
        }

        let state = Self {
            jig_id,
            module_id,
            pair_lookup,
            all_cards,
            game_cards: MutableVec::new(),
            found_cards: MutableVec::new(),
            hover_cards: MutableVec::new(),
            theme_id: raw.theme_id,
            flip_state: Mutable::new(FlipState::None), 
        };

        state.init_new_game();

        state
    }

    pub async fn evaluate(&self, card_1: FlipCard, card_2: FlipCard) {
        let id_1 = card_1.id;
        let id_2 = card_2.id;

        if self.pair_lookup[id_1] == id_2 {
            let hover_card_1 = self.make_hover_card(card_1, Side::Left);
            let hover_card_2 = self.make_hover_card(card_2, Side::Right);
            let mut hover_cards = self.hover_cards.lock_mut();
            hover_cards.push_cloned(hover_card_1);
            hover_cards.push_cloned(hover_card_2);
            let game_cards = self.game_cards.lock_ref();
            if let Some(card) = game_cards.iter().find(|c| c.id == id_1) {
                card.found.set(true);
            }
            if let Some(card) = game_cards.iter().find(|c| c.id == id_2) {
                card.found.set(true);
            }
            /* removing upsets the grid...
            self.game_cards.lock_mut().retain(|card| {
                card.id != id_1 && card.id != id_2
            });
            */
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

    pub fn card_click(&self, id: usize, element:HtmlElement) {
        let flip_state = &mut *self.flip_state.lock_mut();

        let flip_card = FlipCard { id, element};

        match flip_state {
            FlipState::None => *flip_state = FlipState::One(flip_card),
            FlipState::One(other) => {
                if other.id != flip_card.id {
                    *flip_state = FlipState::Two((flip_card, other.clone()))
                }
            },
            _ => {}
        }
    }

    pub fn move_animation_finished(&self, card:&HoverCard) {
        self.hover_cards.lock_mut().retain(|c| c.id != card.id);
        self.found_cards.lock_mut().push_cloned(FoundCard::new(&card));
    }

    pub fn init_new_game(&self) {
        let mut rng = thread_rng();
        let mut cards:Vec<GameCard> = self.all_cards.iter().map(|card| {
            GameCard {
                id: card.id,
                text: card.text.to_string(),
                found: Mutable::new(false)
            }
        }).collect();
        if debug::settings().shuffle {
            cards.shuffle(&mut rng);
        }
        self.game_cards.lock_mut().replace_cloned(cards);
        self.found_cards.lock_mut().clear();
        self.hover_cards.lock_mut().clear();
    }
}

