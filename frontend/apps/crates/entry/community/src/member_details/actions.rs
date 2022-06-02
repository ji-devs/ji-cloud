use std::rc::Rc;

use super::MemberDetails;

impl MemberDetails {
    pub fn load_member(self: &Rc<Self>) {
        // let state = self;

        // state.loader.load(clone!(state => async move {
        //     let path = endpoints::member::Get::PATH.replace("{id}", &state.member_id.0.to_string());
        //     match api_no_auth::<Member, EmptyError, ()>(
        //         &path,
        //         endpoints::member::Get::METHOD,
        //         None
        //     ).await {
        //         Ok(member) => {
        //             state.member.set(Some(member));
        //         },
        //         Err(_) => todo!(),
        //     }
        // }));
    }
}
