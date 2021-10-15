use futures_signals::signal::Mutable;
use rand::prelude::*;
use shared::domain::jig::module::body::matching::Content;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Settings {
    pub rng: Rc<RefCell<ThreadRng>>,
    pub n_choices: Mutable<u8>,
    pub swap: Mutable<bool>,
    pub n_rounds: Mutable<u32>,
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
            time_limit: Mutable::new(
                settings
                    .time_limit
                    .unwrap_or(crate::config::DEFAULT_TIME_LIMIT),
            ),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
        }
    }
}
