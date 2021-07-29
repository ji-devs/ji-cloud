use std::cell::RefCell;
use std::rc::Rc;
use crate::base::state::Base;
use shared::domain::jig::module::body::tapping_board::{Next, Hint};
use super::state::State;

impl State {
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

    pub fn set_next_value(&self, amount: usize) {
        self.base.play_settings.next_value.set(amount);


        if std::mem::discriminant(&*self.base.play_settings.next.lock_ref())
            == std::mem::discriminant(&Next::SelectSome(0)) {
                self.set_next_some();
            }
    }

    pub fn set_next_some(&self) {
        self.set_next(Next::SelectSome(self.base.play_settings.next_value.get()));
    }
}
