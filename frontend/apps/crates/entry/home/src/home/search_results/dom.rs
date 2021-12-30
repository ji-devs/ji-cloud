

use dominator::{html, Dom};


use std::rc::Rc;


use super::state::SearchResults;

impl SearchResults {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("home-search-results", {
            .property_signal("resultsCount", state.total_results_count_signal())
            .property("query", &state.query)
            .child(state.jigs.render())
            .child(state.resources.render())
        })
    }
}
