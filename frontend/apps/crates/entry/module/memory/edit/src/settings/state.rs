use crate::state::Base;
use std::rc::Rc;
use shared::domain::jig::module::body::memory::Content;
use futures_signals::signal::Mutable;

pub struct Settings {
    pub time_limit: Mutable<Option<u32>>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let settings = content.player_settings;

        Self {
            time_limit: Mutable::new(settings.time_limit),
        }
    }
}
