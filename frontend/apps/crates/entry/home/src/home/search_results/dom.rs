use dominator::{html, Dom};
use futures_signals::{signal::SignalExt, map_ref};
use std::rc::Rc;
use crate::home::search_results::search_results_section::SearchResultsSection;

use super::state::SearchResults;

impl SearchResults {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        let search_results_signal = |section: Rc<SearchResultsSection>| {
            // Ensure that jigs and resources are rendered until all requests have completed
            let should_render_signal = map_ref! {
                let loading = state.loading.signal_cloned(),
                let total = section.total.signal_cloned()
                    => {
                        !*loading && *total > 0
                    }
            };

            // Map the search results render call outside of map_ref so that we don't capture
            // `section` inside the closure.
            should_render_signal.map(move |should_render| {
                if should_render {
                    Some(section.render())
                } else {
                    None
                }
            })
        };

        html!("home-search-results", {
            .property_signal("loading", state.loading.signal())
            .property_signal("resultsCount", state.total_results_count_signal())
            .property("query", &state.query)
            .child_signal(search_results_signal(state.jigs.clone()))
            .child_signal(search_results_signal(state.resources.clone()))
        })
    }
}
