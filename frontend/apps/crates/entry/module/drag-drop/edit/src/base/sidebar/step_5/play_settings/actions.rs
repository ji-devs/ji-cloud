use std::cell::RefCell;
use std::rc::Rc;
use crate::base::state::Base;
use shared::domain::jig::module::body::drag_drop::{Next, Hint};
use super::state::PlaySettingsState;

impl PlaySettingsState {
    pub fn set_hint(&self, hint: Hint) {
        self.base.play_settings.hint.set(hint.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.hint = hint;
            }
        })
    }
    pub fn set_next(&self, next: Next) {
        self.base.play_settings.next.set(next.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.next = next;
            }
        })
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
