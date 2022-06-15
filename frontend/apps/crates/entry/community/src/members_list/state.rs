use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::user::public_user::PublicUser;

pub struct MembersList {
    pub items_per_page: u32,
    pub members: Mutable<Option<Vec<PublicUser>>>,
    pub loader: AsyncLoader,
    pub total_pages: Mutable<u32>,
    pub active_page: Mutable<u32>,
}

impl MembersList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            items_per_page: 20,
            members: Mutable::new(None),
            loader: AsyncLoader::new(),
            total_pages: Mutable::new(0),
            active_page: Mutable::new(1),
        })
    }
}
