use std::rc::Rc;

use utils::routes::{CommunityRoute, CommunitySearchQuery, Route};

use crate::state::Community;

impl Community {
    pub fn on_search_click(self: &Rc<Self>) {
        let query = CommunitySearchQuery {
            q: self.q.get_cloned(),
        };
        Route::Community(CommunityRoute::Search(Box::new(query))).go_to();
    }
}
