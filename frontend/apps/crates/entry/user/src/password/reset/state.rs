use std::rc::Rc;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use dominator_helpers::futures::AsyncLoader;
use super::super::state::*;

pub struct PasswordResetPage {
    pub token: String,
    pub loader: AsyncLoader,
    pub password: PasswordState
}

impl PasswordResetPage {
    pub fn new(token: String) -> Rc<Self> {
        Rc::new(Self {
            token,
            loader: AsyncLoader::new(),
            password: PasswordState::new(),
        })
    }

}
