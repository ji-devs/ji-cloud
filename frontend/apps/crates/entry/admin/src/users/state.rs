use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use utils::routes::AdminUsersRoute;

use super::editable_user::EditableUser;

pub struct Users {
    pub route: Mutable<AdminUsersRoute>,
    pub users: MutableVec<Rc<EditableUser>>,
    pub fetch_mode: RefCell<FetchMode>,
    pub loader: AsyncLoader,
    pub active_page: Mutable<u32>,
    pub total_pages: Mutable<Option<u32>>,
}

impl Users {
    pub fn new(route: AdminUsersRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            users: MutableVec::new(),
            fetch_mode: RefCell::new(FetchMode::Browse),
            loader: AsyncLoader::new(),
            active_page: Mutable::new(0),
            total_pages: Mutable::new(None),
        })
    }
}

#[derive(Clone, Debug)]
pub enum FetchMode {
    Browse,
    Search(String),
}
