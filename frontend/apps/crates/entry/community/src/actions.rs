use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::user::public_user::{
        BrowsePublicUserFollowersPath, BrowsePublicUserFollowersQuery,
        BrowsePublicUserFollowingPath, BrowsePublicUserFollowingsQuery,
    },
};
use utils::{
    prelude::{get_user_id, ApiEndpointExt},
    routes::{CommunityRoute, CommunitySearchQuery, Route},
};

use crate::state::Community;

impl Community {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_members_followers(),
                state.load_members_following()
            );
        }));
    }

    async fn load_members_followers(self: &Rc<Self>) {
        let state = self;

        if let Some(user_id) = get_user_id() {
            let req = BrowsePublicUserFollowersQuery {
                page_limit: Some(100),
                ..Default::default()
            };

            let res = endpoints::user::BrowseFollowers::api_no_auth(
                BrowsePublicUserFollowersPath(user_id),
                Some(req),
            )
            .await;
            match res {
                Ok(res) => {
                    state
                        .followers
                        .set(Some(res.followers.iter().map(|u| u.id).collect()));
                }
                Err(_) => todo!(),
            }
        }
    }

    async fn load_members_following(self: &Rc<Self>) {
        let state = self;

        if let Some(user_id) = get_user_id() {
            let req = BrowsePublicUserFollowingsQuery {
                page_limit: Some(100),
                ..Default::default()
            };

            let res = endpoints::user::BrowseFollowing::api_no_auth(
                BrowsePublicUserFollowingPath(user_id),
                Some(req),
            )
            .await;
            match res {
                Ok(res) => {
                    state
                        .followings
                        .set(Some(res.followings.iter().map(|u| u.id).collect()));
                }
                Err(_) => todo!(),
            }
        }
    }

    pub fn on_search_click(self: &Rc<Self>) {
        let query = CommunitySearchQuery {
            q: self.q.get_cloned(),
        };
        dominator::routing::go_to_url(
            &Route::Community(CommunityRoute::Search(Box::new(query))).to_string(),
        );
    }
}
