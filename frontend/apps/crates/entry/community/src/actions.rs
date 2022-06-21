use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::user::public_user::{
        BrowsePublicUserFollowersQuery, BrowsePublicUserFollowersResponse,
        BrowsePublicUserFollowingResponse, BrowsePublicUserFollowingsQuery,
    },
    error::EmptyError,
};
use utils::prelude::{api_no_auth, get_user};

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

        if let Some(user) = get_user() {
            let req = BrowsePublicUserFollowersQuery {
                page_limit: Some(100),
                ..Default::default()
            };

            let path =
                endpoints::user::BrowseFollowers::PATH.replace("{user_id}", &user.id.to_string());
            let res = api_no_auth::<
                BrowsePublicUserFollowersResponse,
                EmptyError,
                BrowsePublicUserFollowersQuery,
            >(&path, endpoints::user::BrowseFollowers::METHOD, Some(req))
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

        if let Some(user) = get_user() {
            let req = BrowsePublicUserFollowingsQuery {
                page_limit: Some(100),
                ..Default::default()
            };

            let path =
                endpoints::user::BrowseFollowing::PATH.replace("{user_id}", &user.id.to_string());
            let res = api_no_auth::<
                BrowsePublicUserFollowingResponse,
                EmptyError,
                BrowsePublicUserFollowingsQuery,
            >(&path, endpoints::user::BrowseFollowers::METHOD, Some(req))
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
}
