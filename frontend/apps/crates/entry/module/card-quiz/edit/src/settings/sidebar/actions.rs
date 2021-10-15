use super::state::*;

impl SidebarSettings {
    pub fn set_n_choices(&self, n_choices: u8) {
        self.base.extra.settings.n_choices.set_neq(n_choices);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.n_choices = n_choices;
            }
        })
    }

    pub fn toggle_swap(&self) {
        let swap = !self.base.extra.settings.swap.get();

        self.base.extra.settings.swap.set_neq(swap);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.swap = swap;
            }
        })
    }

    pub fn set_n_rounds(&self, n_rounds: u32) {
        self.base.extra.settings.n_rounds.set_neq(n_rounds);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                content.player_settings.n_rounds = n_rounds;
            }
        })
    }

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
    pub fn set_has_attempts_limit(&self, flag: bool) {
        self.base.extra.settings.has_attempts_limit.set_neq(flag);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                if !flag {
                    content.player_settings.n_attempts = None;
                } else {
                    let value = self.base.extra.settings.attempts_limit.get();
                    content.player_settings.n_attempts = Some(value);
                }
            }
        })
    }
    pub fn set_attempts_limit(&self, n_attempts: u8) {
        self.base.extra.settings.attempts_limit.set_neq(n_attempts);

        if self.base.extra.settings.has_attempts_limit.get() {
            self.base.history.push_modify(|raw| {
                if let Some(content) = &mut raw.content {
                    content.player_settings.n_attempts = Some(n_attempts);
                }
            })
        }
    }
}
