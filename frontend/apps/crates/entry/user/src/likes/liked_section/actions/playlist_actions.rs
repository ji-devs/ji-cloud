use std::rc::Rc;

use futures_signals::signal_vec::MutableVec;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        playlist::{ListLikedPath, ListLikedRequest, PlaylistId, PlaylistUnlikePath},
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, prelude::ApiEndpointExt, unwrap::UnwrapJiExt};

use crate::likes::liked_section::LikedSection;

impl LikedSection {
    pub async fn load_playlists(self: &Rc<Self>) {
        let req = ListLikedRequest {
            page: Some(self.next_page.get()),
            ..Default::default()
        };
        let res = endpoints::playlist::ListLiked::api_with_auth(ListLikedPath(), Some(req))
            .await
            .toast_on_err();
        let res = bail_on_err!(res);

        let mut playlists = self.list.lock_mut();
        res.playlists.into_iter().for_each(|playlist| {
            playlists.push_cloned(Rc::new(playlist.into()));
        });

        self.total.set(res.total_playlist_count);

        let mut last_page_loaded = self.next_page.lock_mut();
        *last_page_loaded += 1;
    }

    pub(super) async fn unlike_playlist(
        self: &Rc<Self>,
        playlist_id: PlaylistId,
    ) -> Rc<MutableVec<Rc<Asset>>> {
        endpoints::playlist::Unlike::api_with_auth(PlaylistUnlikePath(playlist_id), None)
            .await
            .unwrap_ji();
        Rc::clone(&self.list)
    }
}
