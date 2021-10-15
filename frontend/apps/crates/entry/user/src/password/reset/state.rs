use std::rc::Rc;

use super::super::state::*;
use dominator_helpers::futures::AsyncLoader;

pub struct PasswordResetPage {
    pub token: String,
    pub loader: AsyncLoader,
    pub password: PasswordState,
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
