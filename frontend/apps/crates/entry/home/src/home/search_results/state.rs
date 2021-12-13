use std::rc::{Rc, Weak};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::{Mutable, Signal}, signal_vec::{MutableVec, SignalVecExt}, map_ref};
use shared::domain::jig::{JigResponse, JigSearchResponse, JigFocus, JigId};
use utils::unwrap::UnwrapJiExt;

use super::{super::state::State as HomeState, search_results_section::SearchResultsSection};

#[derive(Clone)]
pub struct SearchResults {
    pub query: String,
    pub jigs: Rc<SearchResultsSection>,
    pub resources: Rc<SearchResultsSection>,
    _home_state: Weak<HomeState>,
}

impl SearchResults {
    pub fn new(home_state: &Rc<HomeState>) -> Rc<Self> {
        let query = home_state.search_selected.query.get_cloned();

        Rc::new(Self {
            query,
            jigs: SearchResultsSection::new(
                JigFocus::Modules,
                Rc::clone(&home_state.search_options),
                Rc::clone(&home_state.search_selected),
                home_state.play_jig.clone(),
            ),
            resources: SearchResultsSection::new(
                JigFocus::Resources,
                Rc::clone(&home_state.search_options),
                Rc::clone(&home_state.search_selected),
                home_state.play_jig.clone(),
            ),
            _home_state: Rc::downgrade(&home_state),
        })
    }

    pub fn home_state(&self) -> Rc<HomeState> {
        // should always be here since parent is the one holding on to search results
        self._home_state.upgrade().unwrap_ji()
    }

    pub fn total_results_count_signal(self: &Rc<Self>) -> impl Signal<Item = u64> {
        map_ref! {
            let resources = self.resources.total.signal(),
            let jig = self.jigs.total.signal() => move {
                resources + jig
            }
        }
    }
}
