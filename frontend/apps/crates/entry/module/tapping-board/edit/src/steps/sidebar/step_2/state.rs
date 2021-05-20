use crate::steps::state::{Step, Base};
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};
use dominator::clone;

pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step2 {
    pub fn new(base: Rc<Base>) -> Self {

        let kind = match crate::debug::settings().content_tab {
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
    Image,
    Audio 
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    //Image(Rc<ImageSearchState>),
    Text(()),
    Image(()),
    Audio(())
}

impl Tab {
    pub fn new(kind:TabKind) -> Self {
        match kind {
            TabKind::Text=> {
                Self::Text(())
            },
            TabKind::Image=> {
                Self::Image(())
            },
            TabKind::Audio => {
                Self::Audio(())
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text(_) => TabKind::Text,
            Self::Image(_) => TabKind::Image,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

