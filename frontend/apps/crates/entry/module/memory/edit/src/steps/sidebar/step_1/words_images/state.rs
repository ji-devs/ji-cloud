use crate::data::state::State as AppState;
use crate::debug::DebugContentTab;
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use dominator::clone;

#[derive(Clone)]
pub struct State {
    pub app: Rc<AppState>,
    pub tab: Mutable<Tab>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tab {
    Text,
    Images
}

impl From<Option<DebugContentTab>> for Tab {
    fn from(tab:Option<DebugContentTab>) -> Self {
        match tab {
            Some(tab) => {
                match tab {
                    DebugContentTab::Text => Self::Text,
                    DebugContentTab::Images => Self::Images
                }
            },
            None => Self::Text
        }
    }
}

impl State {
    pub fn new(app: Rc<AppState>) -> Self {
        let tab = Mutable::new(crate::debug::settings().content_tab.into());

        *app.image_card_click_callback.borrow_mut() = Some(Box::new(clone!(tab => move || {
            tab.set(Tab::Images);
        })));
        Self {
            app,
            tab,
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        *self.app.image_card_click_callback.borrow_mut() = None; 
    }
}
