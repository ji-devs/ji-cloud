use std::rc::{Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::{Mutable, Signal}, signal_vec::{MutableVec, SignalVecExt}, map_ref};
use shared::domain::jig::{JigResponse, JigSearchResponse, JigFocus, JigId};


use crate::home::state::SearchSelected;


use super::super::super::state::SearchOptions;

pub struct SearchResultsSection {
    pub focus: JigFocus,
    pub loader: AsyncLoader,
    pub list: Rc<MutableVec<JigResponse>>,
    pub total: Mutable<u64>,
    pub next_page: Mutable<u32>,
    pub total_pages: Mutable<u32>,
    pub search_options: Rc<SearchOptions>,
    pub search_selected: Rc<SearchSelected>,
    pub play_jig: Mutable<Option<JigId>>,
}

impl SearchResultsSection {
    pub fn new(
        focus: JigFocus,
        search_options: Rc<SearchOptions>,
        search_selected: Rc<SearchSelected>,
        play_jig: Mutable<Option<JigId>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            focus,
            loader: AsyncLoader::new(),
            list: Rc::new(MutableVec::new()),
            total: Mutable::new(0),
            next_page: Mutable::new(0),
            total_pages: Mutable::new(0),
            search_options,
            search_selected,
            play_jig,
        })
    }

    pub fn fill_from_response(self: &Rc<Self>, res: JigSearchResponse) {
        let mut jigs = self.list.lock_mut();
        res.jigs.into_iter().for_each(|jig| {
            jigs.push_cloned(jig);
        });

        self.total.set(res.total_jig_count);

        self.total_pages.set(res.pages);

        let mut last_page_loaded = self.next_page.lock_mut();
        *last_page_loaded += 1;
    }

    pub fn all_loaded_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let list_len = self.list.signal_vec_cloned().len(),
            let total = self.total.signal() => move {
                *list_len == *total as usize
            }
        }
    }
}
