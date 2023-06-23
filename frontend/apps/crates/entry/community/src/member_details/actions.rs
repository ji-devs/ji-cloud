use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        asset::{DraftOrLive, UserOrMe},
        circle::{CircleBrowsePath, CircleBrowseQuery},
        course::{CourseBrowsePath, CourseBrowseQuery},
        jig::{JigBrowsePath, JigBrowseQuery},
        playlist::{PlaylistBrowsePath, PlaylistBrowseQuery},
        resource::{ResourceBrowsePath, ResourceBrowseQuery},
        user::{
            public_user::{
                BrowsePublicUserFollowersPath, BrowsePublicUserFollowersQuery,
                BrowsePublicUserFollowingPath, BrowsePublicUserFollowingsQuery, PublicUser,
                PublicUserFollowPath, PublicUserGetPath, PublicUserUnfollowPath,
            },
            PatchProfilePath, PatchProfileRequest, UserProfile,
        },
    },
};
use utils::{
    location::Country,
    prelude::{get_user_mutable, ApiEndpointExt},
    unwrap::UnwrapJiExt,
};

use super::MemberDetails;

impl MemberDetails {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.load_batch_1().await;
            state.load_batch_2().await;
            state.load_batch_3().await;
        }));
    }

    async fn load_batch_1(self: &Rc<Self>) {
        self.load_member().await;
    }

    async fn load_batch_2(self: &Rc<Self>) {
        join!(
            self.load_members_jigs(),
            self.load_members_playlists(),
            self.load_members_resources(),
            self.load_members_courses(),
        );
    }

    async fn load_batch_3(self: &Rc<Self>) {
        join!(
            self.load_members_circles(),
            self.load_members_following(),
            self.load_members_followers(),
        );
    }

    async fn load_member(self: &Rc<Self>) {
        let state = self;

        match endpoints::user::GetPublicUser::api_no_auth(PublicUserGetPath(state.member_id), None)
            .await
        {
            Ok(member) => {
                state.is_following.set(Some(member.following));
                state.member.set(Some(member));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_members_circles(self: &Rc<Self>) {
        let state = self;

        let req = CircleBrowseQuery {
            users: vec![state.member_id],
            ..Default::default()
        };

        match endpoints::circle::Browse::api_no_auth(CircleBrowsePath(), Some(req)).await {
            Ok(res) => {
                state.circles.set(Some(res.circles));
                state.circles_count.set(Some(res.total_circle_count));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_members_jigs(self: &Rc<Self>) {
        let state = self;

        let req = JigBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        let res = endpoints::jig::Browse::api_no_auth(JigBrowsePath(), Some(req))
            .await
            .unwrap_ji();
        state.jigs.set(Some(res.jigs));
        state.jigs_count.set(Some(res.total_jig_count));
    }

    async fn load_members_playlists(self: &Rc<Self>) {
        let state = self;

        let req = PlaylistBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        let res = endpoints::playlist::Browse::api_no_auth(PlaylistBrowsePath(), Some(req))
            .await
            .unwrap_ji();
        state.playlists.set(Some(res.playlists));
        state.playlists_count.set(Some(res.total_playlist_count));
    }

    async fn load_members_resources(self: &Rc<Self>) {
        let state = self;

        let req = ResourceBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        let res = endpoints::resource::Browse::api_no_auth(ResourceBrowsePath(), Some(req))
            .await
            .unwrap_ji();
        state.resources.set(Some(res.resources));
        state.resources_count.set(Some(res.total_resource_count));
    }

    async fn load_members_courses(self: &Rc<Self>) {
        let state = self;

        let req = CourseBrowseQuery {
            author_id: Some(UserOrMe::User(state.member_id.0)),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        let res = endpoints::course::Browse::api_no_auth(CourseBrowsePath(), Some(req))
            .await
            .unwrap_ji();
        state.courses.set(Some(res.courses));
        state.courses_count.set(Some(res.total_course_count));
    }

    async fn load_members_followers(self: &Rc<Self>) {
        let state = self;

        let req = BrowsePublicUserFollowersQuery {
            ..Default::default()
        };

        let res = endpoints::user::BrowseFollowers::api_no_auth(
            BrowsePublicUserFollowersPath(state.member_id),
            Some(req),
        )
        .await
        .unwrap_ji();
        state.followers.set(Some(res.followers));
        state.followers_count.set(Some(res.total_follower_count));
    }

    async fn load_members_following(self: &Rc<Self>) {
        let state = self;

        let req = BrowsePublicUserFollowingsQuery {
            ..Default::default()
        };

        let res = endpoints::user::BrowseFollowing::api_no_auth(
            BrowsePublicUserFollowingPath(state.member_id),
            Some(req),
        )
        .await
        .unwrap_ji();
        state.following.set(Some(res.followings));
        state.following_count.set(Some(res.total_following_count));
    }

    pub fn follow_member(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let res = endpoints::user::Follow::api_with_auth_empty(
                PublicUserFollowPath(state.member_id),
                None
            ).await;
            match res {
                Ok(_) => {
                    state.is_following.set(Some(true));
                },
                Err(_) => todo!(),
            }
        }));
    }

    pub fn unfollow_member(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let res = endpoints::user::Unfollow::api_with_auth_empty(
                PublicUserUnfollowPath(state.member_id),
                None
            ).await;
            match res {
                Ok(_) => {
                    state.is_following.set(Some(false));
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
                languages_spoken: Some(updated_profile.languages_spoken.clone()),
                languages_spoken_public: Some(updated_profile.languages_spoken_public.clone()),
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

            let res = endpoints::user::PatchProfile::api_with_auth_empty(PatchProfilePath(), Some(req)).await;
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
    let location = Country::from_google_location(&user.location);
    PublicUser {
        id: user.id,
        username: user.username,
        given_name: user.given_name,
        family_name: user.family_name,
        profile_image: user.profile_image,
        organization: user.organization,
        circles: user.circles,
        bio: Some(user.bio),
        badge: user.badge,
        languages_spoken: Some(user.languages_spoken),
        persona: Some(user.persona),
        country_short: location.as_ref().map(|location| location.code.clone()),
        country_long: location.as_ref().map(|location| location.name.clone()),
        jig_count: None,
        resource_count: None,
        course_count: None,
        playlist_count: None,
        total_asset_count: 0, // TODO: bad idea
        following: false,
    }
}
