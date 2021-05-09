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
    Image,
    Color,
    Overlay
}

impl Tab {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Color => "color",
            Self::Overlay => "overlay",
        }
    }
}


impl State {
    pub fn new(app: Rc<AppState>) -> Self {
        let tab = Mutable::new(match crate::debug::settings().bg_tab {
            Some(tab) => tab,
            None => Tab::Image
        });

        Self {
            app,
            tab,
        }
    }
}
