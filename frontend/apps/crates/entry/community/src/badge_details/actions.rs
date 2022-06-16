use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::badge::Badge,
    error::EmptyError,
};
use utils::prelude::api_no_auth;

use super::BadgeDetails;

impl BadgeDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            join!(
                state.load_badge(),
                state.load_badge_members(),
            );
        }));
    }

    async fn load_badge(self: &Rc<Self>) {
        let state = self;

        let path = endpoints::badge::Get::PATH.replace("{id}", &state.badge_id.0.to_string());
        match api_no_auth::<Badge, EmptyError, ()>(&path, endpoints::badge::Get::METHOD, None).await
        {
            Ok(badge) => {
                state.badge.set(Some(badge));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_badge_members(self: &Rc<Self>) {
        // let state = self;

        // let path = endpoints::badge::BrowseMembers::PATH.replace("{id}", &state.badge_id.0.to_string());
        // match api_no_auth::<BrowseMembersResponse, EmptyError, ()>(
        //     &path,
        //     endpoints::badge::BrowseMembers::METHOD,
        //     None
        // ).await {
        //     Ok(res) => {
        //         state.members.lock_mut().extend(res.members);
        //     },
        //     Err(_) => todo!(),
        // }
    }
}
