use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{circle::Circle, user::public_user::PublicUser};
use utils::routes::CommunitySearchQuery;
use web_sys::HtmlElement;

pub const SEARCH_PAGE_LIMIT: u32 = 6;

pub struct CommunitySearch {
    pub members: MutableVec<PublicUser>,
    pub member_count: Mutable<u32>,
    pub circles: MutableVec<Circle>,
    pub circle_count: Mutable<u32>,
    pub loader: AsyncLoader,
    pub query: CommunitySearchQuery,
    pub members_el: RefCell<Option<HtmlElement>>,
    pub circles_el: RefCell<Option<HtmlElement>>,
}

impl CommunitySearch {
    pub fn new(query: CommunitySearchQuery) -> Rc<Self> {
        Rc::new(Self {
            members: MutableVec::new(),
            member_count: Mutable::new(0),
            circles: MutableVec::new(),
            circle_count: Mutable::new(0),
            loader: AsyncLoader::new(),
            query,
            members_el: Default::default(),
            circles_el: Default::default(),
        })
    }
}
