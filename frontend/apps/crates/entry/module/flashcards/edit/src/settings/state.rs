use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::module::body::flashcards::{Content, DisplayMode};
use std::cell::RefCell;

pub struct Settings {
    pub display_mode: Mutable<DisplayMode>,
    pub view_all: Mutable<bool>,
    pub view_pairs: Mutable<u32>,
    pub swap: Mutable<bool>,
    pub rng: RefCell<ThreadRng>,
}

const DEFAULT_VIEW_PAIRS: u32 = 4;

impl Settings {
    pub fn new(content: Content) -> Self {
        let player_settings = content.player_settings;

        Self {
            view_pairs: Mutable::new(player_settings.view_pairs.unwrap_or(DEFAULT_VIEW_PAIRS)),
            view_all: Mutable::new(player_settings.view_pairs.is_none()),
            display_mode: Mutable::new(player_settings.display_mode),
            swap: Mutable::new(player_settings.swap),
            rng: RefCell::new(thread_rng()),
        }
    }
}
