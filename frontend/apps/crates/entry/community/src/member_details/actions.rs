use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        asset::{DraftOrLive, UserOrMe},
        jig::{JigBrowseQuery, JigFocus},
        user::public_user::{
            BrowsePublicUserFollowersQuery, BrowsePublicUserFollowersResponse,
            BrowsePublicUserFollowingResponse, BrowsePublicUserFollowingsQuery, PublicUser,
        },
    },
    error::EmptyError,
};
use utils::prelude::{api_no_auth, api_with_auth_empty, ApiEndpointExt};

use super::{Connections, Creations, MemberDetails};

impl MemberDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_member(),
                state.load_members_jigs(),
                state.load_members_followers(),
            );
        }));
    }

    async fn load_member(self: &Rc<Self>) {
        let state = self;

        let path =
            endpoints::user::GetPublicUser::PATH.replace("{user_id}", &state.member_id.to_string());
        match api_no_auth::<PublicUser, EmptyError, ()>(
            &path,
            endpoints::user::GetPublicUser::METHOD,
            None,
        )
        .await
        {
            Ok(member) => {
                state.member.set(Some(member));
            }
            Err(_) => todo!(),
        }
    }

    pub fn set_active_creations(self: &Rc<Self>, creations: Creations) {
        let state = self;
        state.creations.set(creations.clone());
        state.loader.load(clone!(state => async move {
            match creations {
                Creations::Jigs(_) => state.load_members_jigs().await,
                Creations::Resources(_) => state.load_members_resources().await,
            };
        }));
    }

    async fn load_members_jigs(self: &Rc<Self>) {
        let state = self;

        let req = JigBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            jig_focus: Some(JigFocus::Modules),
            ..Default::default()
        };

        match endpoints::jig::Browse::api_no_auth(Some(req)).await {
            Ok(res) => state.creations.set(Creations::Jigs(Some(res.jigs))),
            Err(_) => todo!(),
        }
    }

    async fn load_members_resources(self: &Rc<Self>) {
        let state = self;

        let req = JigBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            jig_focus: Some(JigFocus::Resources),
            ..Default::default()
        };

        match endpoints::jig::Browse::api_no_auth(Some(req)).await {
            Ok(res) => state.creations.set(Creations::Resources(Some(res.jigs))),
            Err(_) => todo!(),
        }
    }

    pub fn set_active_connections(self: &Rc<Self>, connections: Connections) {
        let state = self;
        state.connections.set(connections.clone());
        state.loader.load(clone!(state => async move {
            match connections {
                Connections::Followers(_) => state.load_members_followers().await,
                Connections::Following(_) => state.load_members_following().await,
            };
        }));
    }

    async fn load_members_followers(self: &Rc<Self>) {
        let state = self;

        let req = BrowsePublicUserFollowersQuery {
            ..Default::default()
        };

        let path = endpoints::user::BrowseFollowers::PATH
            .replace("{user_id}", &state.member_id.to_string());
        let res = api_no_auth::<
            BrowsePublicUserFollowersResponse,
            EmptyError,
            BrowsePublicUserFollowersQuery,
        >(&path, endpoints::user::BrowseFollowers::METHOD, Some(req))
        .await;
        match res {
            Ok(res) => {
                state
                    .connections
                    .set(Connections::Followers(Some(res.followers)));

                // state.followers.lock_mut().extend(res.followers);
            }
            Err(_) => todo!(),
        }
    }

    async fn load_members_following(self: &Rc<Self>) {
        let state = self;

        let req = BrowsePublicUserFollowingsQuery {
            ..Default::default()
        };

        let path = endpoints::user::BrowseFollowing::PATH
            .replace("{user_id}", &state.member_id.to_string());
        let res = api_no_auth::<
            BrowsePublicUserFollowingResponse,
            EmptyError,
            BrowsePublicUserFollowingsQuery,
        >(&path, endpoints::user::BrowseFollowers::METHOD, Some(req))
        .await;
        match res {
            Ok(res) => {
                state
                    .connections
                    .set(Connections::Following(Some(res.followings)));
            }
            Err(_) => todo!(),
        }
    }

    pub fn follow_member(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let path = endpoints::user::Follow::PATH.replace("{user_id}", &state.member_id.to_string());
            let res = api_with_auth_empty::<EmptyError, ()>(
                &path,
                endpoints::user::Follow::METHOD,
                None
            ).await;
            match res {
                Ok(_) => {
                    let mut followings = state.community_state.followings.lock_mut();
                    if let Some(followings) = &mut *followings {
                        followings.push(state.member_id);
                    }
                },
                Err(_) => todo!(),
            }
        }));
    }

    pub fn unfollow_member(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let path = endpoints::user::Unfollow::PATH.replace("{user_id}", &state.member_id.to_string());
            let res = api_with_auth_empty::<EmptyError, ()>(
                &path,
                endpoints::user::Unfollow::METHOD,
                None
            ).await;
            match res {
                Ok(_) => {
                    let mut followings = state.community_state.followings.lock_mut();
                    if let Some(followings) = &mut *followings {
                        let index = followings.iter().position(|followee| followee == &state.member_id);
                        if let Some(index) = index {
                            followings.remove(index);
                        }
                    }
                },
                Err(_) => todo!(),
            }
        }));
    }
}
