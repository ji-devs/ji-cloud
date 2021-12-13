
use components::module::_common::thumbnail::ModuleThumbnail;
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::jig::{JigResponse, JigFocus};
use std::rc::Rc;
use utils::{ages::AgeRangeVecExt, events, jig::published_at_string};

use super::state::SearchResults;

const STR_LOAD_MORE: &str = "See more";

impl SearchResults {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("home-search-results", {
            .property_signal("resultsCount", state.total_results_count_signal())
            .property("query", &state.query)
            .child(state.jigs.render())
            // .child(state.resources.render()) // TODO:
        })
    }
}
