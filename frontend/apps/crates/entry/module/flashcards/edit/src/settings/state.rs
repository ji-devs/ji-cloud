use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::jig::module::body::flashcards::{Content, DisplayMode};
use std::cell::RefCell;

pub struct Settings {
    pub display_mode: Mutable<DisplayMode>,
    pub rng: RefCell<ThreadRng>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        Self {
            display_mode: Mutable::new(content.player_settings.display_mode),
            rng: RefCell::new(thread_rng()),
        }
    }
}
