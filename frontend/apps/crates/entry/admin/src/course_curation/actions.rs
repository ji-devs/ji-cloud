use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::domain::course::OrderBy;
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        course::{
            CourseBrowsePath, CourseBrowseQuery, CourseResponse, CourseSearchPath,
            CourseSearchQuery,
        },
        meta::GetMetadataPath,
    },
};
use utils::{
    editable_asset::EditableCourse,
    prelude::ApiEndpointExt,
    routes::{AdminCourseCurationRoute, AdminRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::{CourseCuration, FetchMode};

impl CourseCuration {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_courses(),
                state.load_meta()
            );
        }));
    }

    async fn load_meta(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.ages.set(meta.age_ranges);
                self.affiliations.set(meta.affiliations);
            }
        };
    }

    pub async fn load_courses(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_courses_browse().await,
            FetchMode::Search(query) => self.load_courses_search(query.clone()).await,
        };

        self.courses.lock_mut().replace_cloned(
            res.courses
                .into_iter()
                .map(|course| Rc::new(course.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_courses_browse(&self) -> CourseListResponse {
        let req = CourseBrowseQuery {
            page: Some(self.active_page.get()),
            draft_or_live: Some(DraftOrLive::Live),
            order_by: Some(self.order_by.get()),
            ..Default::default()
        };

        match endpoints::course::Browse::api_with_auth(CourseBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => CourseListResponse {
                courses: res.courses,
                total_pages: res.pages,
            },
        }
    }

    async fn load_courses_search(&self, query: String) -> CourseListResponse {
        let req = CourseSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::course::Search::api_with_auth(CourseSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => CourseListResponse {
                courses: res.courses,
                total_pages: res.pages,
            },
        }
    }

    pub fn set_order_by(self: &Rc<Self>, order_by: OrderBy) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.order_by.set(order_by);
            state.load_courses().await;
        }));
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_courses().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminCourseCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::CourseCuration(route)).push_state();
    }

    pub fn save_and_publish(self: &Rc<Self>, course: &Rc<EditableCourse>) {
        self.loader.load(clone!(course => async move {
            let (a, b) = join!(
                course.save_draft(),
                course.save_admin_data(),
            );
            a.unwrap_ji();
            b.unwrap_ji();
            course.publish().await.unwrap_ji();
        }))
    }

    pub fn save_admin_data(self: &Rc<Self>, course: &Rc<EditableCourse>) {
        self.loader.load(clone!(course => async move {
            course.save_admin_data().await.unwrap_ji();
        }))
    }
}

#[derive(Clone, Debug)]
struct CourseListResponse {
    courses: Vec<CourseResponse>,
    total_pages: u32,
}
