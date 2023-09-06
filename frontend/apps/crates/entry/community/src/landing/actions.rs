use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::domain::user::UserBadge;
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        circle::{self, CircleBrowsePath, CircleBrowseQuery},
        course::{self, CourseBrowsePath, CourseBrowseQuery},
        user::public_user::{self, PublicUserBrowsePath, UserBrowseQuery},
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
                state.load_top_courses(),
            );
        }));
    }

    async fn load_top_members(self: &Rc<Self>) {
        let state = self;
        let req = UserBrowseQuery {
            page_limit: Some(10),
            order_by: Some(public_user::OrderBy::AssetCount),
            badge: vec![UserBadge::NoBadge, UserBadge::MasterTeacher],
            ..Default::default()
        };

        match endpoints::user::BrowsePublicUser::api_with_auth(PublicUserBrowsePath(), Some(req))
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
            page_limit: Some(5),
            order_by: Some(circle::OrderBy::MemberCount),
            ..Default::default()
        };

        match endpoints::circle::Browse::api_with_auth(CircleBrowsePath(), Some(req)).await {
            Ok(res) => {
                state.top_circles.set(Some(res.circles));
            }
            Err(_) => todo!(),
        }
    }

    async fn load_top_courses(self: &Rc<Self>) {
        let state = self;
        let req = CourseBrowseQuery {
            page_limit: Some(10),
            order_by: Some(course::OrderBy::PlayCount),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        match endpoints::course::Browse::api_with_auth(CourseBrowsePath(), Some(req)).await {
            Ok(res) => {
                state.top_courses.set(Some(res.courses));
            }
            Err(_) => todo!(),
        }
    }
}
