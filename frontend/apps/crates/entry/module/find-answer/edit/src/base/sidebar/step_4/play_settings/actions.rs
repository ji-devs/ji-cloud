use super::state::State;
use shared::domain::module::body::find_answer::{Next, Ordering};

impl State {
    pub fn set_ordering(&self, ordering: Ordering) {
        self.base.play_settings.ordering.set(ordering.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.ordering = ordering;
            }
        })
    }

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

    pub fn set_attempts_limit(&self, n_attempts: u8) {
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

    pub fn set_next(&self, next: Next) {
        self.base.play_settings.next.set(next.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.next = next;
            }
        })
    }

    pub fn set_next_value(&self, amount: usize) {
        self.base.play_settings.next_value.set(amount);

        if std::mem::discriminant(&*self.base.play_settings.next.lock_ref())
            == std::mem::discriminant(&Next::SelectSome(0))
        {
            self.set_next_some();
        }
    }

    pub fn set_next_some(&self) {
        self.set_next(Next::SelectSome(self.base.play_settings.next_value.get()));
    }
}
