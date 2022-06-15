use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{badge::BadgeSearchQuery, user::public_user::SearchPublicUserQuery},
};
use utils::prelude::ApiEndpointExt;

use super::CommunitySearch;

impl CommunitySearch {
    pub fn search(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            join!(
                state.search_members_async(0),
                state.search_badges_async(0),
            );
        }));
    }

    async fn search_members_async(self: &Rc<Self>, page: u32) {
        let state = self;
        let req = SearchPublicUserQuery {
            page: Some(page),
            ..Default::default()
        };

        match endpoints::user::Search::api_no_auth(Some(req)).await {
            Ok(res) => {
                state.members.lock_mut().extend(res.users);
            }
            Err(_) => todo!(),
        }
    }

    async fn search_badges_async(self: &Rc<Self>, page: u32) {
        let state = self;
        let req = BadgeSearchQuery {
            page: Some(page),
            ..Default::default()
        };

        match endpoints::badge::Search::api_no_auth(Some(req)).await {
            Ok(res) => {
                state.badges.lock_mut().extend(res.badges);
            }
            Err(_) => todo!(),
        }
    }
}
