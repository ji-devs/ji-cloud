use std::cell::RefCell;
use std::rc::Rc;

use super::callbacks::Callbacks;
use futures_signals::signal_vec::MutableVecLockRef;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
    signal_vec::{MutableVec, SignalVecExt},
};
use web_sys::HtmlElement;

pub struct State {
    pub left: Rc<MutableVec<Mutable<String>>>,
    pub right: Rc<MutableVec<Mutable<String>>>,
    pub is_placeholder: Mutable<bool>,
    pub error_element_ref: RefCell<Option<HtmlElement>>,
    pub callbacks: Callbacks,
    pub opts: Options,
    pub confirm_clear: Mutable<bool>,
}

pub struct Options {
    /// number of rows to show
    pub max_rows: usize,
    /// number of rows for input within a cell
    pub cell_rows: u8,
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

impl State {
    pub fn new(opts: Options, callbacks: Callbacks) -> Self {
        Self {
            left: Rc::new(MutableVec::new_with_values(
                (0..opts.max_rows)
                    .map(|_| Mutable::new(String::default()))
                    .collect(),
            )),
            right: Rc::new(MutableVec::new_with_values(
                (0..opts.max_rows)
                    .map(|_| Mutable::new(String::default()))
                    .collect(),
            )),
            is_placeholder: Mutable::new(true),
            error_element_ref: RefCell::new(None),
            callbacks,
            opts,
            confirm_clear: Mutable::new(false),
        }
    }

    pub fn derive_list(self: &Rc<Self>) -> Option<Vec<(String, String)>> {
        let into_list = |vec: MutableVecLockRef<Mutable<String>>| {
            vec.iter()
                .map(|value| value.get_cloned())
                .collect::<Vec<String>>()
        };

        // Create the left and right list without filtering the values. We need to be able to work
        // out whether the actual _rows_ have valid data.
        let left = into_list(self.left.lock_ref());
        let right = into_list(self.right.lock_ref());

        self.clone().filtered_list(&left, &right).map(|list| {
            list.iter()
                .map(|(left, right)| (left.to_string(), right.to_string()))
                .collect()
        })
    }

    pub fn is_valid_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        let state = Rc::clone(self);

        let into_signal = |signal_vec: &Rc<MutableVec<Mutable<String>>>| {
            signal_vec
                .signal_vec_cloned()
                .map_signal(|inner| inner.signal_cloned())
                .to_signal_map(|value| {
                    value
                        .iter()
                        .map(|value| value.to_owned())
                        .collect::<Vec<String>>()
                })
        };

        // Create the left and right list without filtering the values. We need to be able to work
        // out whether the actual _rows_ have valid data.
        let left_sig = into_signal(&state.left);
        let right_sig = into_signal(&state.right);

        map_ref! {
            let left = left_sig,
            let right = right_sig
                => move {
                    state.clone().filtered_list(left, right).is_some()
                }
        }
    }

    /// Compares the left and right lists and returns a zipped list _if_ left and right have the same
    /// amount of values and their lengths are >= `min_valid` and <= `max_rows`.
    ///
    /// Note: an owned Rc<Self> is required because a caller can be inside a signals callback and
    /// we need to guarantee that `self` remains valid.
    fn filtered_list<'a>(
        self: Rc<Self>,
        left: &'a Vec<String>,
        right: &'a Vec<String>,
    ) -> Option<Vec<(&'a str, &'a str)>> {
        // We need to recreate the iterator because it is consumed
        let create_iter = |left: &'a Vec<String>, right: &'a Vec<String>| {
            left.iter()
                .zip(right.iter())
                // Make sure that we won't be checking whether strings like "   " are empty. This also
                // cleans up strings wrapped with whitespace.
                .map(|(left, right)| (left.trim(), right.trim()))
        };

        // First ensure that each row has values for left and right. If we did a filter instead,
        // then the filter would simply _remove_ invalid values which would be fine, but it
        // would affect the UX - If a teacher added 6 words for the left, and only 5 on the right
        // by accident, they'd have to recapture their entries again to correct the mistake.
        let contents_valid = create_iter(left, right)
            // Ensure that each row has both left and right values set.
            // Because both lists are prefilled with empty strings, we need to check that either
            // side is empty, but not both sides as that would be an empty row.
            .find(|(left, right)| {
                (left.is_empty() || right.is_empty()) && !(left.is_empty() && right.is_empty())
            })
            .is_none();

        if contents_valid {
            let list: Vec<(&str, &str)> = create_iter(left, right)
                // At this point we know that the only rows with empty data are the prefilled rows,
                // so we can safely filter them out.
                .filter(|(left, right)| !left.is_empty() && !right.is_empty())
                .collect();
            // If the final list's length is less than the minimum or somehow there are more rows, the list
            // is invalid.
            let list_len = list.len();
            if list_len >= self.opts.min_valid && list_len <= self.opts.max_rows {
                Some(list)
            } else {
                None
            }
        } else {
            None
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
