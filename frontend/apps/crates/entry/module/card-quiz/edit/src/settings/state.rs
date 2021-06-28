use crate::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use rand::prelude::*;
use shared::domain::jig::module::body::card_quiz::Content;
use futures_signals::signal::Mutable;
use components::module::_groups::cards::lookup::Side;

pub struct Settings {
    pub rng: Rc<RefCell<ThreadRng>>,
    pub n_choices: Mutable<u8>,
    pub swap: Mutable<bool>,
    pub n_rounds: Mutable<u32>,
    pub n_attempts: Mutable<Option<u8>>,
    pub time_limit: Mutable<Option<u32>>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let settings = content.player_settings;

        Self {
            rng: Rc::new(RefCell::new(thread_rng())),
            n_choices: Mutable::new(settings.n_choices),
            swap: Mutable::new(settings.swap),
            n_rounds: Mutable::new(settings.n_rounds),
            n_attempts: Mutable::new(settings.n_attempts),
            time_limit: Mutable::new(settings.time_limit),
        }
    }
}
