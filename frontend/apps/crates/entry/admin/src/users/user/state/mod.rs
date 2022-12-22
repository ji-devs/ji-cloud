use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{user::{UserId, public_user::PublicUser}};
use std::{rc::Rc};

use crate::users::{Users};


pub struct AdminUser {
    pub user_id: UserId,
    pub user: Rc<PublicUser>,
    pub loader: AsyncLoader,
    pub users_state: Rc<Users>,
    pub player_open: Mutable<bool>,
}

impl AdminUser {
    pub fn new(users_state: Rc<Users>, user_id: UserId, user: Rc<PublicUser>) -> Rc<Self> {
        Rc::new(Self {
            user_id,
            user,
            loader: AsyncLoader::new(),
            users_state,
            player_open: Mutable::new(false),
        })
    }
}
