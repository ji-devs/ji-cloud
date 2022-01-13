use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use crate::home::search_results::search_results_section::SearchResultsSection;

use super::state::SearchResults;

impl SearchResults {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        let search_results_signal = |state: Rc<SearchResultsSection>| {
            state.total.signal_cloned().map(clone!(state => move |total| {
                if total > 0 {
                    Some(state.render())
                } else {
                    None
                }
            }))
        };

        html!("home-search-results", {
            .property_signal("resultsCount", state.total_results_count_signal())
            .property("query", &state.query)
            .child_signal(search_results_signal(state.jigs.clone()))
            .child_signal(search_results_signal(state.resources.clone()))
        })
    }
}
