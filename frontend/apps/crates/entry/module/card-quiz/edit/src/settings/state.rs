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
    pub attempts_limit: Mutable<u8>,
    pub has_attempts_limit: Mutable<bool>,
    pub time_limit: Mutable<u32>,
    pub has_time_limit: Mutable<bool>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let settings = content.player_settings;

        Self {
            rng: Rc::new(RefCell::new(thread_rng())),
            n_choices: Mutable::new(settings.n_choices),
            swap: Mutable::new(settings.swap),
            n_rounds: Mutable::new(settings.n_rounds),
            attempts_limit: Mutable::new(settings.n_attempts.unwrap_or(crate::config::DEFAULT_ATTEMPTS_LIMIT)),
            has_attempts_limit: Mutable::new(settings.n_attempts.is_some()),
            time_limit: Mutable::new(settings.time_limit.unwrap_or(crate::config::DEFAULT_TIME_LIMIT)),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
        }
    }
}
