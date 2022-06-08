use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal_vec::MutableVec;
use shared::domain::user::public_user::PublicUser;

pub struct MembersList {
    pub members: MutableVec<PublicUser>,
    pub loader: AsyncLoader,
}

impl MembersList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            members: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
