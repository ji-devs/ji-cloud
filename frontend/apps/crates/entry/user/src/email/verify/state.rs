use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator_helpers::futures::AsyncLoader;

pub struct VerifyEmailPage {
    pub token: String,
    pub mode: Mutable<Mode>,
    pub loader: AsyncLoader,
}

impl VerifyEmailPage {
    pub fn new(token: String) -> Rc<Self> {
        Rc::new(Self {
            token,
            mode: Mutable::new(Mode::Verifying),
            loader: AsyncLoader::new()
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode { 
    Verifying,
    Success,
    Error
}
