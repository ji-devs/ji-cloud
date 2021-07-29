use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator_helpers::futures::AsyncLoader;

pub struct VerifyEmailPage {
    pub token: String,
}

impl VerifyEmailPage {
    pub fn new(token: String) -> Rc<Self> {
        Rc::new(Self {
            token,
        })
    }
}
