use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        course::{CourseBrowsePath, CourseBrowseQuery, OrderBy},
    },
};
use utils::prelude::ApiEndpointExt;

use super::CoursesList;

impl CoursesList {
    pub fn load_courses(self: &Rc<Self>) {
        let state = self;

        state.courses.set(None);

        state.loader.load(clone!(state => async move {
            let req = CourseBrowseQuery {
                page: Some(state.active_page.get() - 1),
                page_limit: Some(state.items_per_page),
                order_by: Some(OrderBy::PlayCount),
                draft_or_live: Some(DraftOrLive::Live),
                ..Default::default()
            };

            match endpoints::course::Browse::api_with_auth(CourseBrowsePath(), Some(req)).await {
                Ok(res) => {
                    state.courses.set(Some(res.courses));
                    let page_count = page_count(res.total_course_count as u32, state.items_per_page);
                    state.total_pages.set(page_count);
                    state.total_course_count.set(Some(res.total_course_count as u32))
                },
                Err(_) => todo!(),
            }
        }));
    }
}

fn page_count(total: u32, items_per_page: u32) -> u32 {
    let total = total as f32;
    let items_per_page = items_per_page as f32;
    let page_count = total / items_per_page;
    let page_count = page_count.ceil();
    page_count as u32
}
