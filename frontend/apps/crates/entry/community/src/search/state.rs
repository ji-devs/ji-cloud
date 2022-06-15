use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::{badge::Badge, user::public_user::PublicUser};
use utils::routes::CommunitySearchQuery;

pub struct CommunitySearch {
    pub members: MutableVec<PublicUser>,
    pub badges: MutableVec<Badge>,
    pub loader: AsyncLoader,
}

impl CommunitySearch {
    pub fn new(_search: CommunitySearchQuery) -> Rc<Self> {
        Rc::new(Self {
            members: MutableVec::new(),
            badges: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
