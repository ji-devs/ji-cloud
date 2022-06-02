use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;

pub struct MemberDetails {
    // pub member_id: MemberId,
    // pub member: Mutable<Option<Member>>,
    pub loader: AsyncLoader,
}

impl MemberDetails {
    pub fn new(/*member_id: MemberId*/) -> Rc<Self> {
        Rc::new(Self {
            // member_id,
            // member: Mutable::new(None),
            loader: AsyncLoader::new(),
        })
    }
}
