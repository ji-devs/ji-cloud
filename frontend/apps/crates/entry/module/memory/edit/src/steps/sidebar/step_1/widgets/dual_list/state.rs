use std::rc::Rc;
use wasm_bindgen::prelude::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use crate::data::state::{State as AppState, GameMode};
use web_sys::HtmlElement;

pub struct State {
    pub app: Rc<AppState>,
    pub left: Rc<MutableVec<Mutable<String>>>,
    pub right: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>,
    pub error_element_ref: Mutable<Option<HtmlElement>>,
    pub error: Mutable<Option<Error>>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NumWords
}

impl Error {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NumWords => crate::strings::error::STR_SINGLE_LIST_NUM_WORDS
        }
    }
}

type IsPlaceholder = bool;

impl State {
    pub fn new(app: Rc<AppState>, max:usize) -> Self {
        Self {
            app,
            left: Rc::new(MutableVec::new_with_values(
                    (0..max)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            )),
            right: Rc::new(MutableVec::new_with_values(
                    (0..max)
                        .map(|_| Mutable::new(String::default()))
                        .collect()
            )),
            is_placeholder: Mutable::new(true),
            error_element_ref: Mutable::new(None),
            error: Mutable::new(None),
        }
    }

    pub fn error_signal(&self) -> impl Signal<Item = Option<(Error, HtmlElement)>> {
        map_ref! {
            let err = self.error.signal(),
            let elem = self.error_element_ref.signal_cloned()
                => {
                    match (err, elem) {
                        (Some(err), Some(elem)) => Some((*err, elem.clone())),
                        _ => None
                    }
                }
        }
    }
    pub fn derive_list(&self) -> Result<Vec<(String, String)>, Error> {
        let list:Vec<(String, String)> = 
        self.left
            .lock_ref()
            .iter()
            .map(|mutable_string| {
                mutable_string.get_cloned()
            })
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .zip(
                self.right
                    .lock_ref()
                    .iter()
                    .map(|mutable_string| {
                        mutable_string.get_cloned()
                    })
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
            )
            .collect();

        if list.len() < 2 {
            Err(Error::NumWords)
        } else {
            Ok(list)
        }

    }
    pub fn clear(&self) {
        for mutable_string in self.left.lock_ref().iter() {
            mutable_string.set(String::default());
        }
        for mutable_string in self.right.lock_ref().iter() {
            mutable_string.set(String::default());
        }

        self.is_placeholder.set_neq(true);
    }

}

