use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        circle::{CircleSearchPath, CircleSearchQuery},
        user::public_user::{PublicUserSearchPath, SearchPublicUserQuery},
    },
};
use utils::prelude::ApiEndpointExt;

use super::{CommunitySearch, SEARCH_PAGE_LIMIT};

impl CommunitySearch {
    pub fn search(self: &Rc<Self>) {
        let state = self;

        state.loader.load(clone!(state => async move {
            join!(
                state.search_members_async(0),
                state.search_circles_async(0),
            );
        }));
    }

    pub fn load_more_members(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let member_len = state.members.lock_ref().len() as u32;
            let next_page = member_len / SEARCH_PAGE_LIMIT;
            state.search_members_async(next_page).await;
        }));
    }

    pub fn load_more_circles(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            let circle_len = state.circles.lock_ref().len() as u32;
            let next_page = circle_len / SEARCH_PAGE_LIMIT;
            state.search_circles_async(next_page).await;
        }));
    }

    async fn search_members_async(self: &Rc<Self>, page: u32) {
        let state = self;
        let req = SearchPublicUserQuery {
            q: state.query.q.clone(),
            page: Some(page),
            page_limit: Some(SEARCH_PAGE_LIMIT),
            ..Default::default()
        };

        match endpoints::user::Search::api_with_auth(PublicUserSearchPath(), Some(req)).await {
            Ok(res) => {
                state.members.lock_mut().extend(res.users);
                state.member_count.set_neq(res.total_user_count as u32);
            }
            Err(_) => todo!(),
        }
    }

    async fn search_circles_async(self: &Rc<Self>, page: u32) {
        let state = self;
        let req = CircleSearchQuery {
            q: state.query.q.clone(),
            page: Some(page),
            page_limit: Some(SEARCH_PAGE_LIMIT),
            ..Default::default()
        };

        match endpoints::circle::Search::api_with_auth(CircleSearchPath(), Some(req)).await {
            Ok(res) => {
                state.circles.lock_mut().extend(res.circles);
                state.circle_count.set_neq(res.total_circle_count as u32);
            }
            Err(_) => todo!(),
        }
    }
}
