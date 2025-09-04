use crate::home::actions::search;
use crate::home::search_results::search_results_section::SearchResultsSection;
use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use std::rc::Rc;

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
            .prop_signal("loading", state.loading.signal())
            .prop_signal("jigCount", state.jigs.total.signal())
            .prop_signal("playlistCount", state.playlists.total.signal())
            .prop_signal("resourceCount", state.resources.total.signal())
            .prop("isRated", state.rated_only)
            .prop("query", &state.query)
            .child(state.jigs.home_state.search_bar.render_rated_toggle(Rc::new(clone!(state => move || {
                search(&state.jigs.home_state)
            })), Some("rated")))
            .child_signal(search_results_signal(Rc::clone(&state.playlists)))
            .child_signal(search_results_signal(Rc::clone(&state.jigs)))
            .child_signal(search_results_signal(Rc::clone(&state.resources)))
        })
    }
}
