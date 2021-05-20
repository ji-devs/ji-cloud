use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;

pub struct Step3 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Self {

        let kind = match crate::debug::settings().interaction_tab {
            Some(kind) => kind,
            None => TabKind::Text
        };

        let tab = Mutable::new(Tab::new(kind));

        Self {
            base,
            tab
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Audio 
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Audio => "audio",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    //Image(Rc<ImageSearchState>),
    Text(()),
    Audio(())
}

impl Tab {
    pub fn new(kind:TabKind) -> Self {
        match kind {
            TabKind::Text=> {
                Self::Text(())
            },
            TabKind::Audio => {
                Self::Audio(())
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text(_) => TabKind::Text,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

