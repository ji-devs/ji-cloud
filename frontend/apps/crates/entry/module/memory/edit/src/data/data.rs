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
use crate::pages::all_modes::card_dom::*;

#[derive(Clone, Debug)]
pub struct Theme {
    pub id: &'static str,
    pub label: &'static str,
}
#[derive(Clone, Copy, PartialEq)]
pub enum ContentMode {
    Text,
    Images
}

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

    pub fn set_from_loaded(&self, step: usize, raw_game_state:raw::GameState) {
        if self.mode.get().is_some() {
            panic!("setting the game state from loaded only works on first-load!");
        }

        let (mode, state) = match raw_game_state {
            raw::GameState::Duplicate(raw_state) => {
                let mode = GameMode::Duplicate;
                (
                    Some(mode),
                    Some(BaseGameState::from_raw(step, mode, raw_state, self.jig_id.clone(), self.module_id.clone()))
                )
            },
            raw::GameState::WordsAndImages(raw_state) => {
                let mode = GameMode::WordsAndImages;
                (
                    Some(mode),
                    Some(BaseGameState::from_raw(step, mode, raw_state, self.jig_id.clone(), self.module_id.clone()))
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameMode {
    Duplicate,
    WordsAndImages,
}

type Id = String;

#[derive(Clone, Debug)]
pub enum Card {
    Text(Mutable<String>),
    Image(Mutable<Option<Id>>),
    Audio(Mutable<Option<Id>>),
}


impl Card {
    pub fn new_text(text:String) -> Self {
        Self::Text(Mutable::new(text))
    }
    pub fn new_image(id:Option<Id>) -> Self {
        Self::Image(Mutable::new(id))
    }
    pub fn new_audio(id:Option<Id>) -> Self {
        Self::Audio(Mutable::new(id))
    }

    pub fn set_text(&self, text:String) {
        match self {
            Self::Text(m) => m.set_neq(text),
            _ => panic!("not a text card!")
        }
    }
    pub fn set_img(&self, id:Option<Id>) {
        match self {
            Self::Image(m) => m.set_neq(id),
            _ => panic!("not an image card!")
        }
    }
    pub fn set_audio(&self, id:Option<Id>) {
        match self {
            Self::Audio(m) => m.set_neq(id),
            _ => panic!("not an audio card!")
        }
    }
}

impl From<raw::Card> for Card {
    fn from(card:raw::Card) -> Self {
        match card {
            raw::Card::Text(text) => Card::new_text(text),
            raw::Card::Image(id) => Card::new_image(id),
            raw::Card::Audio(id) => Card::new_audio(id),
        }
    }
}

impl From<&Card> for raw::Card {
    fn from(card:&Card) -> Self {
        match card {
            Card::Text(text) => raw::Card::Text(text.get_cloned()),
            Card::Image(id) => raw::Card::Image(id.get_cloned()),
            Card::Audio(id) => raw::Card::Audio(id.get_cloned())
        }
    }
}


#[derive(Debug)]
pub struct BaseGameState {
    pub mode: GameMode,
    pub jig_id: String,
    pub module_id: String,
    pub step: Mutable<Step>,
    pub pairs: MutableVec<(Card, Card)>,
    pub theme_id: Mutable<String>,
    pub edit_text_list: MutableVec<String>
}

impl BaseGameState {

    pub fn cards_preview_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.pairs
            .signal_vec_cloned()
            .enumerate()
            .map(clone!(_self => move |(index, (card_1, card_2))| {
                CardPairPreviewDom::render(CardPairPreviewDom::new(_self.clone(), index, card_1, card_2))
            }))
    }

    pub fn cards_edit_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.pairs
            .signal_vec_cloned()
            .enumerate()
            .map(clone!(_self => move |(index, (card_1, card_2))| {
                CardPairEditDom::render(CardPairEditDom::new(_self.clone(), index, card_1, card_2))
            }))
    }
    pub fn words_signal(&self) -> impl Signal<Item = Vec<String>> {
        self.edit_text_list
            .signal_vec_cloned()
            .to_signal_cloned()
    }

    pub fn text_input_signal(&self) -> impl Signal<Item = String> {
        self.edit_text_list
            .signal_vec_cloned()
            .to_signal_map(|x| x.join("\n"))
    }

    pub fn to_raw(&self) -> raw::BaseGameState {
        raw::BaseGameState {
            pairs: self.pairs.lock_ref()
                .iter()
                .map(|(card_1, card_2)| 
                    (card_1.into(), card_2.into())
                )
                .collect(),
            theme_id: self.theme_id.get_cloned(),
        }
    }

    pub fn from_raw(step: usize, mode: GameMode, raw_game_state: raw::BaseGameState, jig_id: String, module_id: String) -> Self {

        #[derive(Clone, Copy, PartialEq)]
        enum CardNumber {
            One,
            Two
        }
        let mut pairs:Vec<(Card, Card)> = Vec::new();
        let mut edit_text_list:Vec<String> = Vec::new();
        
        let mut derive_edit_data = |card:&raw::Card, num:CardNumber| {
            match card {
                raw::Card::Text(text) => {
                    if num == CardNumber::One || mode != GameMode::Duplicate {
                        edit_text_list.push(text.to_string());
                    }
                },
                _ => {}
            }
        };

        for (card_1, card_2) in raw_game_state.pairs.into_iter() {
            derive_edit_data(&card_1, CardNumber::One);
            derive_edit_data(&card_2, CardNumber::Two);
            pairs.push((card_1.into(), card_2.into()));
        }

        Self {
            mode,
            jig_id,
            module_id,
            step: Mutable::new(step.into()),
            pairs: MutableVec::new_with_values(pairs),
            theme_id: Mutable::new(raw_game_state.theme_id),
            edit_text_list: MutableVec::new_with_values(edit_text_list),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Step {
    One,
    Two,
    Three,
    Four
}

impl From<usize> for Step {
    fn from(step:usize) -> Self {
        match step {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => panic!("unallowed step!") 
        }
    }
}
