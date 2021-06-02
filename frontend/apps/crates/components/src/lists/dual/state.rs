use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use web_sys::HtmlElement;
use super::callbacks::Callbacks;

pub struct State {
    pub rows:u8,
    pub left: Rc<MutableVec<Mutable<String>>>,
    pub right: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>,
    pub error_element_ref: RefCell<Option<HtmlElement>>,
    pub callbacks: Callbacks,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NumWords
}

impl Error {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NumWords => super::strings::error::STR_NUM_WORDS
        }
    }
}

type IsPlaceholder = bool;

impl State {
    pub fn new(rows: u8, max:usize, callbacks: Callbacks) -> Self {
        Self {
            rows,
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
            error_element_ref: RefCell::new(None),
            callbacks,
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

    pub fn is_ready_signal(&self) -> impl Signal<Item = bool> {
        //TODO - like derive_list?
        futures_signals::signal::always(true)
    }

}

