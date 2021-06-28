use crate::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;
use shared::domain::jig::module::body::flashcards::{DisplayMode, Content};
use futures_signals::signal::Mutable;

pub struct Settings {
    pub display_mode: Mutable<DisplayMode>,
    pub rng: RefCell<ThreadRng>,
}

impl Settings {
    pub fn new(content:Content) -> Self {
        Self {
            display_mode: Mutable::new(content.player_settings.display_mode),
            rng: RefCell::new(thread_rng())
        }
    }
}
