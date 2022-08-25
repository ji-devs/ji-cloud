use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::{
        asset::{DraftOrLive, UserOrMe},
        circle::CircleBrowseQuery,
        jig::JigBrowseQuery,
        resource::ResourceBrowseQuery,
        user::{
            public_user::{
                BrowsePublicUserFollowersQuery, BrowsePublicUserFollowersResponse,
                BrowsePublicUserFollowingResponse, BrowsePublicUserFollowingsQuery, PublicUser,
            },
            PatchProfileRequest, UserProfile,
        },
    },
    error::EmptyError,
};
use utils::prelude::{api_no_auth, api_with_auth_empty, get_user_mutable, ApiEndpointExt};

use super::{Connections, Creations, MemberDetails};

impl MemberDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_member(),
                state.load_circles(),
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

    async fn load_circles(self: &Rc<Self>) {
        let state = self;

        let req = CircleBrowseQuery {
            users: vec![state.member_id],
            ..Default::default()
        };

        match endpoints::circle::Browse::api_no_auth(Some(req)).await {
            Ok(res) => {
                state.circles.set(res.circles);
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
            ..Default::default()
        };

        match endpoints::jig::Browse::api_no_auth(Some(req)).await {
            Ok(res) => state.creations.set(Creations::Jigs(Some(res.jigs))),
            Err(_) => todo!(),
        }
    }

    async fn load_members_resources(self: &Rc<Self>) {
        let state = self;

        let req = ResourceBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        match endpoints::resource::Browse::api_no_auth(Some(req)).await {
            Ok(res) => state
                .creations
                .set(Creations::Resources(Some(res.resources))),
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

    pub fn save_profile_changes(self: &Rc<Self>, updated_profile: UserProfile) {
        let state = self;
        state.active_popup.set(None);
        state.loader.load(clone!(state => async move {
            let req = PatchProfileRequest {
                username: Some(updated_profile.username.clone()),
                given_name: Some(updated_profile.given_name.clone()),
                family_name: Some(updated_profile.family_name.clone()),
                profile_image: Some(updated_profile.profile_image.clone()),
                bio: Some(updated_profile.bio.clone()),
                language_app: Some(updated_profile.language_app.clone()),
                language_emails: Some(updated_profile.language_emails.clone()),
                language_spoken: Some(updated_profile.language_spoken.clone()),
                language_spoken_public: Some(updated_profile.language_spoken_public.clone()),
                timezone: Some(updated_profile.timezone.clone()),
                opt_into_edu_resources: Some(updated_profile.opt_into_edu_resources.clone()),
                organization_public: Some(updated_profile.organization_public),
                persona_public: Some(updated_profile.persona_public),
                location_public: Some(updated_profile.location_public),
                bio_public: Some(updated_profile.bio_public),
                organization: Some(updated_profile.organization.clone()),
                persona: Some(updated_profile.persona.clone()),
                subjects: Some(updated_profile.subjects.clone()),
                age_ranges: Some(updated_profile.age_ranges.clone()),
                affiliations: Some(updated_profile.affiliations.clone()),
                location: Some(updated_profile.location.clone()),
            };

            let res = endpoints::user::PatchProfile::api_with_auth_empty(Some(req)).await;
            if let Err(_err) = res {
                todo!()
            }
            get_user_mutable().set(Some(updated_profile.clone()));
            let public_user = user_to_public_user(updated_profile);
            state.member.set(Some(public_user))
        }));
    }
}

fn user_to_public_user(user: UserProfile) -> PublicUser {
    // includes fields not marked as public
    PublicUser {
        id: user.id,
        username: user.username,
        given_name: user.given_name,
        family_name: user.family_name,
        profile_image: user.profile_image,
        organization: user.organization,
        circles: user.circles,
        bio: Some(user.bio),
        language_spoken: Some(user.language_spoken),
        persona: Some(user.persona),
        location: user.location,
    }
}
