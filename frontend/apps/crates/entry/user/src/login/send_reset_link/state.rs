use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use crate::email_handler::EmailHandler;

use super::SendResetLinkCallbacks;

pub struct SendResetLink {
    pub(super) reset_sent: Mutable<bool>,
    pub email: EmailHandler,
    pub loader: AsyncLoader,
    pub callbacks: SendResetLinkCallbacks,
}

impl SendResetLink {
    pub fn new(callbacks: SendResetLinkCallbacks) -> Rc<Self> {
        Rc::new(Self {
            reset_sent: Mutable::new(false),
            email: EmailHandler::new(),
            loader: AsyncLoader::new(),
            callbacks,
        })
    }
}
