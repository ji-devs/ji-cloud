use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;

pub struct MembersList {
    // pub members: MutableVec<Member>,
    pub loader: AsyncLoader,
}

impl MembersList {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            // members: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }
}
