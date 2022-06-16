use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::badge::Badge,
    error::EmptyError,
};
use utils::prelude::{api_no_auth, api_with_auth_empty};

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

    pub fn join_badge(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let path = endpoints::badge::JoinBadge::PATH.replace("{id}", &state.badge_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::badge::JoinBadge::METHOD, None).await
            {
                Ok(_) => {
                    let mut user = state.community_state.user.get_cloned();
                    user.badges.push(state.badge_id);
                    state.community_state.user.set(user);
                }
                Err(_) => todo!(),
            }
        }));
    }

    pub fn leave_badge(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let path = endpoints::badge::LeaveBadge::PATH.replace("{id}", &state.badge_id.0.to_string());
            match api_with_auth_empty::<EmptyError, ()>(&path, endpoints::badge::LeaveBadge::METHOD, None).await
            {
                Ok(_) => {
                    let mut user = state.community_state.user.get_cloned();
                    let index = user.badges.iter().position(|badge| *badge == state.badge_id).unwrap();
                    user.badges.remove(index);
                }
                Err(_) => todo!(),
            }
        }));
    }
}
