use std::rc::Rc;

use super::{HebrewButtons, Popup};

impl HebrewButtons {
    pub(super) fn on_action_click(self: &Rc<Self>, popup: Popup) {
        let mut active_popup = self.active_popup.lock_mut();

        *active_popup = match &*active_popup {
            Some(active_popup) if active_popup == &popup => None,
            _ => Some(popup),
        };
    }
}
