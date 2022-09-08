use std::rc::Rc;

use super::{HebrewButtonOpened, HebrewButtons, Popup};
use dominator::traits::StaticEvent;
use web_sys::{window, Event};

impl HebrewButtons {
    pub(super) fn on_action_click(self: &Rc<Self>, popup: Popup) {
        let _ = window()
            .unwrap()
            .dispatch_event(&Event::new(HebrewButtonOpened::EVENT_TYPE).unwrap());

        let mut active_popup = self.active_popup.lock_mut();

        *active_popup = match &*active_popup {
            Some(active_popup) if active_popup == &popup => None,
            _ => Some(popup),
        };
    }
}
