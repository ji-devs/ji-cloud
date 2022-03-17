use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::memory::Content;

pub struct Settings {
    pub time_limit: Mutable<u32>,
    pub has_time_limit: Mutable<bool>,
    pub use_default_pairs: Mutable<bool>,
    pub pairs_to_display: Mutable<u32>,
}

impl Settings {
    pub fn new(content: Content) -> Self {
        let settings = content.player_settings;

        Self {
            time_limit: Mutable::new(
                settings
                    .time_limit
                    .unwrap_or(crate::config::DEFAULT_TIME_LIMIT),
            ),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
            use_default_pairs: Mutable::new(settings.pairs_to_display.is_none()),
            pairs_to_display: Mutable::new(settings.pairs_to_display.unwrap_or_default()),
        }
    }
}
