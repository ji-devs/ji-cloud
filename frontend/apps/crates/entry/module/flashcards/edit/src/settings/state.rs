use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::jig::module::body::flashcards::{Content, DisplayMode};
use std::cell::RefCell;

pub struct Settings {
    pub display_mode: Mutable<DisplayMode>,
    pub swap: Mutable<bool>,
    pub rng: RefCell<ThreadRng>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let player_settings = content.player_settings;

        Self {
            display_mode: Mutable::new(player_settings.display_mode),
            swap: Mutable::new(player_settings.swap),
            rng: RefCell::new(thread_rng()),
        }
    }
}
