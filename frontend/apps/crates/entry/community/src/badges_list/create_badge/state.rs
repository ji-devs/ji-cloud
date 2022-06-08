use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;

use crate::badges_list::BadgesList;

pub struct CreateBadge {
    pub loader: AsyncLoader,
    pub name: RefCell<String>,
    pub description: RefCell<String>,
    pub badge_list_state: Rc<BadgesList>,
}

impl CreateBadge {
    pub fn new(badge_list_state: Rc<BadgesList>) -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            name: RefCell::new(String::new()),
            description: RefCell::new(String::new()),
            badge_list_state,
        })
    }
}
