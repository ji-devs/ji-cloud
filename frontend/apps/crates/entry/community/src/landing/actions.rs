use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        circle::{CircleBrowsePath, CircleBrowseQuery},
        user::public_user::{PublicUserBrowsePath, UserBrowseQuery},
    },
};
use utils::prelude::ApiEndpointExt;

use super::CommunityLanding;

impl CommunityLanding {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_top_members(),
                state.load_top_circles(),
            );
        }));
    }

    async fn load_top_members(self: &Rc<Self>) {
        let state = self;
        let req = UserBrowseQuery {
            page_limit: Some(6),
            ..Default::default()
        };

        match endpoints::user::BrowsePublicUser::api_no_auth(PublicUserBrowsePath(), Some(req))
            .await
        {
            Ok(res) => {
                state.top_members.set(Some(res.users));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_top_circles(self: &Rc<Self>) {
        let state = self;
        let req = CircleBrowseQuery {
            page_limit: Some(6),
            ..Default::default()
        };

        match endpoints::circle::Browse::api_no_auth(CircleBrowsePath(), Some(req)).await {
            Ok(res) => {
                state.top_circles.set(Some(res.circles));
            }
            Err(_) => todo!(),
        }
    }
}
