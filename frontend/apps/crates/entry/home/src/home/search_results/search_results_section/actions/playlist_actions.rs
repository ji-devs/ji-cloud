use std::rc::Rc;

use shared::{api::endpoints, domain::playlist::PlaylistSearchPath};
use utils::prelude::ApiEndpointExt;

use crate::home::search_results::search_results_section::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_playlists(self: &Rc<Self>) {
        let mut req = self.home_state.search_bar.get_search_request_playlist();

        req.page = Some(self.next_page.get());

        match endpoints::playlist::Search::api_with_auth(PlaylistSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => {
                let mut playlists = self.list.lock_mut();
                res.playlists.into_iter().for_each(|playlist| {
                    playlists.push_cloned(Rc::new(playlist.into()));
                });

                self.total.set(res.total_playlist_count);

                let mut last_page_loaded = self.next_page.lock_mut();
                *last_page_loaded += 1;
            }
        };
    }
}
