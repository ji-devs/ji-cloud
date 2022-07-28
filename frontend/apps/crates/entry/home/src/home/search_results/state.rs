use std::rc::Rc;

use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use shared::domain::asset::AssetType;

use super::{super::state::Home, search_results_section::SearchResultsSection};

#[derive(Clone)]
pub struct SearchResults {
    pub loading: Mutable<bool>,
    pub query: String,
    pub jigs: Rc<SearchResultsSection>,
    pub resources: Rc<SearchResultsSection>,
    pub courses: Rc<SearchResultsSection>,
}

impl SearchResults {
    pub fn new(home_state: &Rc<Home>, loading: bool) -> Rc<Self> {
        let query = home_state.search_selected.query.get_cloned();

        Rc::new(Self {
            loading: Mutable::new(loading),
            query,
            jigs: SearchResultsSection::new(Rc::clone(&home_state), AssetType::Jig),
            resources: SearchResultsSection::new(Rc::clone(&home_state), AssetType::Resource),
            courses: SearchResultsSection::new(Rc::clone(&home_state), AssetType::Course),
        })
    }

    pub fn total_results_count_signal(self: &Rc<Self>) -> impl Signal<Item = u64> {
        map_ref! {
            let resources = self.resources.total.signal(),
            let jigs = self.jigs.total.signal() => move {
                resources + jigs
            }
        }
    }
}
