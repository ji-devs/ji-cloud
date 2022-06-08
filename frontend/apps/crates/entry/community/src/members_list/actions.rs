use std::rc::Rc;

use dominator::clone;
use shared::{api::endpoints, domain::user::public_user::UserBrowseQuery};
use utils::prelude::ApiEndpointExt;

use super::MembersList;

impl MembersList {
    pub fn load_members(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let req = UserBrowseQuery {
                page: None,
                ..Default::default()
            };

            match endpoints::user::BrowsePublicUser::api_no_auth(Some(req)).await {
                Ok(res) => {
                    state.members.lock_mut().extend(res.users);
                },
                Err(_) => todo!(),
            }
        }));
    }
}
