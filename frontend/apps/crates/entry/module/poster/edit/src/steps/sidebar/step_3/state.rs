use crate::data::state::State as AppState;
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
    Image,
    Audio 
}

impl Tab {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
        }
    }
}


impl State {
    pub fn new(app: Rc<AppState>) -> Self {
        let tab = Mutable::new(match crate::debug::settings().content_tab {
            Some(tab) => tab,
            None => Tab::Image
        });

        Self {
            app,
            tab,
        }
    }
}
