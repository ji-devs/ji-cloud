use dominator_helpers::futures::AsyncLoader;
use shared::domain::user::UserId;
use std::rc::Rc;

use crate::users::{editable_user::EditableUser, Users};

pub struct AdminUser {
    pub user_id: UserId,
    pub user: Rc<EditableUser>,
    pub loader: AsyncLoader,
    pub users_state: Rc<Users>,
}

impl AdminUser {
    pub fn new(users_state: Rc<Users>, user_id: UserId, user: Rc<EditableUser>) -> Rc<Self> {
        Rc::new(Self {
            user_id,
            user,
            loader: AsyncLoader::new(),
            users_state,
        })
    }
}
