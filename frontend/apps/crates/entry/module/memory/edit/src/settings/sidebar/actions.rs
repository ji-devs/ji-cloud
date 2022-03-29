use shared::config::{MAX_LIST_WORDS, MIN_LIST_WORDS};

use super::state::*;

impl SidebarSettings {
    pub fn set_has_time_limit(&self, flag: bool) {
        self.base.extra.settings.has_time_limit.set_neq(flag);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                if !flag {
                    content.player_settings.time_limit = None;
                } else {
                    let value = self.base.extra.settings.time_limit.get();
                    content.player_settings.time_limit = Some(value);
                }
            }
        })
    }
    pub fn set_time_limit(&self, time_limit: u32) {
        self.base.extra.settings.time_limit.set_neq(time_limit);

        if self.base.extra.settings.has_time_limit.get() {
            self.base.history.push_modify(|raw| {
                if let Some(content) = &mut raw.content {
                    content.player_settings.time_limit = Some(time_limit);
                }
            })
        }
    }

    pub fn set_default_pairs(&self) {
        let pairs_len = self.base.pairs.lock_ref().len();
        let mut pairs_to_display = if pairs_len > MAX_LIST_WORDS {
            MAX_LIST_WORDS as u32
        } else {
            pairs_len as u32
        };

        if pairs_to_display < MIN_LIST_WORDS as u32 {
            pairs_to_display = MIN_LIST_WORDS as u32;
        }

        self.base
            .extra
            .settings
            .pairs_to_display
            .set_neq(pairs_to_display);
    }

    pub fn toggle_use_default_pairs(&self) {
        let use_default_pairs = !self.base.extra.settings.use_default_pairs.get();
        self.base
            .extra
            .settings
            .use_default_pairs
            .set_neq(use_default_pairs);

        if use_default_pairs {
            self.set_default_pairs();
        }

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.pairs_to_display =
                    if self.base.extra.settings.use_default_pairs.get() {
                        None
                    } else {
                        Some(self.base.extra.settings.pairs_to_display.get())
                    }
            }
        })
    }

    pub fn set_pairs_to_display(&self, pairs_to_display: u32) {
        // Ensure that the entered amount is not greater than the maximum amount of pairs.
        let mut pairs_to_display = if pairs_to_display > MAX_LIST_WORDS as u32 {
            MAX_LIST_WORDS as u32
        } else {
            pairs_to_display
        };

        // Make sure that the amount is not greater than the actual amount of pairs.
        let pairs_len = self.base.pairs.lock_ref().len() as u32;
        if pairs_to_display > pairs_len {
            pairs_to_display = pairs_len
        }

        if pairs_to_display < MIN_LIST_WORDS as u32 {
            pairs_to_display = MIN_LIST_WORDS as u32;
        }

        self.base.extra.settings.use_default_pairs.set_neq(false);
        self.base
            .extra
            .settings
            .pairs_to_display
            .set_neq(pairs_to_display);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.pairs_to_display =
                    if self.base.extra.settings.use_default_pairs.get() {
                        None
                    } else {
                        Some(pairs_to_display)
                    }
            }
        })
    }
}
