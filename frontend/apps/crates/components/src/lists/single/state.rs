use std::cell::RefCell;
use std::rc::Rc;

use super::callbacks::Callbacks;
use futures_signals::{
    signal::{Mutable, Signal},
    signal_vec::{MutableVec, SignalVecExt},
};
use web_sys::HtmlElement;

pub struct State {
    pub list: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>,
    pub error_element_ref: RefCell<Option<HtmlElement>>,
    pub callbacks: Callbacks,
    pub opts: Options,
}

pub struct Options {
    /// number of rows to show
    pub max_rows: usize,
    /// minimum number of valid entries required
    pub min_valid: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NumWords,
}

impl Error {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NumWords => super::strings::error::STR_NUM_WORDS,
        }
    }
}

type IsPlaceholder = bool;

impl State {
    pub fn new(opts: Options, callbacks: Callbacks) -> Self {
        Self {
            list: Rc::new(MutableVec::new_with_values(
                (0..opts.max_rows)
                    .map(|_| Mutable::new(String::default()))
                    .collect(),
            )),
            is_placeholder: Mutable::new(true),
            error_element_ref: RefCell::new(None),
            callbacks,
            opts,
        }
    }

    /// TODO - can derive_list and is_valid_signal be consolidated?
    pub fn derive_list(&self) -> Result<Vec<String>, Error> {
        let lock = self.list.lock_ref();

        let list: Vec<String> = lock
            .iter()
            .map(|mutable_string| mutable_string.get_cloned())
            .filter(|x| !x.is_empty())
            .collect();

        if list.len() < self.opts.min_valid {
            Err(Error::NumWords)
        } else {
            Ok(list)
        }
    }

    pub fn is_valid_signal(&self) -> impl Signal<Item = Result<(), Error>> {
        let min_valid = self.opts.min_valid;

        self.list
            .signal_vec_cloned()
            .map_signal(|inner| inner.signal_cloned())
            .to_signal_map(move |values| {
                let valid_len = values.iter().filter(|x| !x.is_empty()).count();

                if valid_len < min_valid {
                    Err(Error::NumWords)
                } else {
                    Ok(())
                }
            })
    }

    pub fn clear(&self) {
        for mutable_string in self.list.lock_ref().iter() {
            mutable_string.set(String::default());
        }

        self.is_placeholder.set_neq(true);
    }
}
