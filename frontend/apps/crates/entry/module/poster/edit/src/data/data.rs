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
use utils::settings::SETTINGS;


pub struct State {
    pub jig_id: String,
    pub module_id: String,
    pub poster: Rc<Poster>,
    pub tool: Mutable<Tool>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tool {
    Layout,
    Images
}

pub struct Poster {
}

impl Poster {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_from_raw(&self, raw_poster:raw::Poster) {
    }

}

impl State {
    pub fn new(jig_id:String, module_id: String) -> Self {
        Self {
            jig_id,
            module_id,
            poster: Rc::new(Poster::new()),
            tool: Mutable::new(debug::settings().tool)
        }
    }

    pub fn set_from_loaded(&self, raw_poster:raw::Poster) {
        self.poster.set_from_raw(raw_poster);
    }
}

pub struct Layout {
    pub id: &'static str,
    pub label: &'static str,
    thumbnail: &'static str, 
}

impl Layout {
    pub const fn new(id:&'static str, label:&'static str, thumbnail:&'static str) -> Self {
        Self {
            id,
            label,
            thumbnail
        }
    }

    pub fn thumbnail_url(&self) -> String {
        format!("{}/{}",
                unsafe { SETTINGS.get_unchecked().remote_target.media_ui_url() },
                self.thumbnail
        )
    }
}

pub struct Theme {
    pub id:&'static str,
    pub label:&'static str,
}
