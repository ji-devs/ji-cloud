use std::rc::Rc;

use shared::{api::endpoints, domain::course::CourseSearchPath};
use utils::prelude::ApiEndpointExt;

use crate::home::search_results::search_results_section::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_courses(self: &Rc<Self>) {
        let mut req = self.home_state.search_bar.get_search_request_course();

        req.page = Some(self.next_page.get());

        match endpoints::course::Search::api_no_auth(CourseSearchPath(), Some(req)).await {
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
