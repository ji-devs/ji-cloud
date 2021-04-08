use crate::data::state::State as AppState;
use crate::debug::DebugContentTab;
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

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
        Self {
            app,
            tab: Mutable::new(crate::debug::settings().content_tab.into())
        }
    }
}


