use std::rc::Rc;

use super::MembersList;

impl MembersList {
    pub fn load_members(self: &Rc<Self>) {
        // let state = self;

        // state.loader.load(clone!(state => async move {
        //     let req = MemberBrowseQuery {
        //         page: None,
        //         ..Default::default()
        //     };

        //     match endpoints::member::Browse::api_no_auth(Some(req)).await {
        //         Ok(member) => {
        //             state.members.lock_mut().extend(member.members);
        //         },
        //         Err(_) => todo!(),
        //     }
        // }));
    }
}
