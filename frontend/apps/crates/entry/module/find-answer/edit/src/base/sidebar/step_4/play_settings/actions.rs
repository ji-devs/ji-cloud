use super::state::State;
use shared::domain::module::body::find_answer::Ordering;

impl State {
    pub fn set_ordering(&self, ordering: Ordering) {
        self.base.play_settings.ordering.set(ordering.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.ordering = ordering;
            }
        })
    }

    // TODO: Once attempt limits functionality has been implemented, remove this `allow`
    #[allow(dead_code)]
    pub fn set_has_attempts_limit(&self, flag: bool) {
        self.base.play_settings.has_attempts_limit.set_neq(flag);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                if !flag {
                    content.play_settings.n_attempts = None;
                } else {
                    let value = self.base.play_settings.n_attempts.get();
                    content.play_settings.n_attempts = Some(value);
                }
            }
        })
    }

    // TODO: Once attempt limits functionality has been implemented, remove this `allow`
    #[allow(dead_code)]
    pub fn set_attempts_limit(&self, n_attempts: u32) {
        self.base.play_settings.n_attempts.set_neq(n_attempts);

        if self.base.play_settings.has_attempts_limit.get() {
            self.base.history.push_modify(|raw| {
                if let Some(content) = &mut raw.content {
                    content.play_settings.n_attempts = Some(n_attempts);
                }
            })
        }
    }

    pub fn set_has_time_limit(&self, flag: bool) {
        self.base.play_settings.has_time_limit.set_neq(flag);

        self.base.history.push_modify(|raw| {
            if let Some(content) = &mut raw.content {
                if !flag {
                    content.play_settings.time_limit = None;
                } else {
                    let value = self.base.play_settings.time_limit.get();
                    content.play_settings.time_limit = Some(value);
                }
            }
        })
    }
    pub fn set_time_limit(&self, time_limit: u32) {
        self.base.play_settings.time_limit.set_neq(time_limit);

        if self.base.play_settings.has_time_limit.get() {
            self.base.history.push_modify(|raw| {
                if let Some(content) = &mut raw.content {
                    content.play_settings.time_limit = Some(time_limit);
                }
            })
        }
    }
}
