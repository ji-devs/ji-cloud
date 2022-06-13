use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{badge::BadgeBrowseQuery, user::public_user::UserBrowseQuery},
};
use utils::prelude::ApiEndpointExt;

use super::CommunityLanding;

impl CommunityLanding {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_top_members(),
                state.load_top_badges(),
            );
        }));
    }

    async fn load_top_members(self: &Rc<Self>) {
        let state = self;
        let req = UserBrowseQuery {
            page_limit: Some(6),
            ..Default::default()
        };

        match endpoints::user::BrowsePublicUser::api_no_auth(Some(req)).await {
            Ok(res) => {
                state.top_members.set(Some(res.users));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_top_badges(self: &Rc<Self>) {
        let state = self;
        let req = BadgeBrowseQuery {
            page_limit: Some(6),
            ..Default::default()
        };

        match endpoints::badge::Browse::api_no_auth(Some(req)).await {
            Ok(res) => {
                state.top_badges.set(Some(res.badges));
            }
            Err(_) => todo!(),
        }
    }
}
