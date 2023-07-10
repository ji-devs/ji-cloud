use std::rc::Rc;

use dominator::clone;

use crate::course_curation::FetchMode;

use super::state::CourseTable;

impl CourseTable {
    pub fn search_courses(self: &Rc<Self>, query: String) {
        let state = self;
        let mut fetch_mode = state.curation_state.fetch_mode.borrow_mut();
        if query.is_empty() {
            *fetch_mode = FetchMode::Browse;
        } else {
            *fetch_mode = FetchMode::Search(query);
        }

        state.curation_state.active_page.set(0);

        state.loader.load(clone!(state => async move {
            state.curation_state.load_courses().await;
        }));
    }
}
