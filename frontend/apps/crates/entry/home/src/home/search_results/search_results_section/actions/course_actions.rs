use std::rc::Rc;

use shared::{
    api::{endpoints, ApiEndpoint},
    domain::course::{CourseSearchQuery, CourseSearchResponse},
    error::EmptyError,
};
use utils::prelude::api_no_auth;

use crate::home::search_results::search_results_section::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_courses(self: &Rc<Self>) {
        let mut req = self.home_state.search_selected.to_course_search_request();

        req.page = Some(self.next_page.get());

        match api_no_auth::<CourseSearchResponse, EmptyError, CourseSearchQuery>(
            endpoints::course::Search::PATH,
            endpoints::course::Search::METHOD,
            Some(req),
        )
        .await
        {
            Err(_) => todo!(),
            Ok(res) => {
                let mut courses = self.list.lock_mut();
                res.courses.into_iter().for_each(|course| {
                    courses.push_cloned(Rc::new(course.into()));
                });

                self.total.set(res.total_course_count);

                let mut last_page_loaded = self.next_page.lock_mut();
                *last_page_loaded += 1;
            }
        };
    }
}
