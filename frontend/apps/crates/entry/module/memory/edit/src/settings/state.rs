use crate::state::Base;
use std::rc::Rc;
use shared::domain::jig::module::body::memory::Content;
use futures_signals::signal::Mutable;

pub struct Settings {
    pub time_limit: Mutable<u32>,
    pub has_time_limit: Mutable<bool>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let settings = content.player_settings;

        Self {
            time_limit: Mutable::new(settings.time_limit.unwrap_or(crate::config::DEFAULT_TIME_LIMIT)),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
        }
    }
}
