use dominator_helpers::futures::AsyncLoader;
use std::rc::Rc;

use crate::users::Users;

pub struct UsersTable {
    pub loader: AsyncLoader,
    pub users_state: Rc<Users>,
}

impl UsersTable {
    pub fn new(users_state: Rc<Users>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            users_state,
        })
    }
}
