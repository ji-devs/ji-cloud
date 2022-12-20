use super::state::State;
use shared::domain::module::body::tapping_board::{Hint, Next};

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
}
