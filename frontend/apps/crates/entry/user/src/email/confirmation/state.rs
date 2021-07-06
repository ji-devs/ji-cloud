use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator_helpers::futures::AsyncLoader;

pub struct SendEmailConfirmationPage {
    pub email: String,
    pub mode: Mutable<Mode>,
    pub loader: AsyncLoader,
}

impl SendEmailConfirmationPage {
    pub fn new(email: String) -> Rc<Self> {
        Rc::new(Self {
            email,
            mode: Mutable::new(Mode::Send),
            loader: AsyncLoader::new()
        })
    }

    pub fn mode_str(&self) -> impl Signal<Item = &'static str> {
        self.mode.signal().map(|mode| mode.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode { 
    Send,
    Sent
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Send => "send",
            Self::Sent => "sent",
        }
    }
}
