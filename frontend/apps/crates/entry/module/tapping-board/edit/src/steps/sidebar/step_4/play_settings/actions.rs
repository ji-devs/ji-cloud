use std::cell::RefCell;
use std::rc::Rc;
use crate::steps::state::Base;
use shared::domain::jig::module::body::tapping_board::{Next, Hint};
use super::state::State;

impl State {
    pub fn on_next_amount_value(&self, amount: usize) {
        *self.some_amount.borrow_mut() = amount;


        if std::mem::discriminant(&*self.base.play_settings.next.lock_ref())
            == std::mem::discriminant(&Next::SelectSome(0)) {
                self.set_next_amount();
            }
    }

    pub fn set_next_amount(&self) {
        let amount = *self.some_amount.borrow();

        self.set_next(Next::SelectSome(amount));
    }
    pub fn set_next(&self, next: Next) {
        self.base.play_settings.next.set(next.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.next = next;
            }
        })
    }
    pub fn set_hint(&self, hint: Hint) {
        self.base.play_settings.hint.set(hint.clone());

        self.base.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.play_settings.hint = hint;
            }
        })
    }
}
