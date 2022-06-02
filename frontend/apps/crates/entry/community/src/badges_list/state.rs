use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::badge::Badge;

pub struct BadgesList {
    pub badges: MutableVec<Badge>,
    pub loader: AsyncLoader,
}

impl BadgesList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            badges: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
