use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{badge::Badge, user::public_user::PublicUser};
use utils::routes::CommunitySearchQuery;

pub const SEARCH_PAGE_LIMIT: u32 = 5;

pub struct CommunitySearch {
    pub members: MutableVec<PublicUser>,
    pub member_count: Mutable<u32>,
    pub badges: MutableVec<Badge>,
    pub badge_count: Mutable<u32>,
    pub loader: AsyncLoader,
}

impl CommunitySearch {
    pub fn new(_search: CommunitySearchQuery) -> Rc<Self> {
        Rc::new(Self {
            members: MutableVec::new(),
            member_count: Mutable::new(0),
            badges: MutableVec::new(),
            badge_count: Mutable::new(0),
            loader: AsyncLoader::new(),
        })
    }
}