use std::rc::Rc;

use dominator::clone;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        jig::{JigFeaturedPath, JigFeaturedUpdateRequest, JigId, JigSearchPath},
        playlist::{
            PlaylistFeaturedPath, PlaylistFeaturedUpdateRequest, PlaylistId, PlaylistSearchPath,
        },
    },
};
use utils::{bail_on_err, error_ext::ErrorExt, fetch::ApiEndpointExt, toasts};
use wasm_bindgen_futures::spawn_local;

use super::{FeaturedAssets, FeaturedKind};

impl FeaturedAssets {
    pub(super) fn search(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            let (assets, total): (Vec<Asset>, u64) = match state.kind {
                FeaturedKind::Jigs => {
                    let mut req = state.search_bar.search_selected.to_jig_search_request();
                    state.active_query.set(req.q.clone());
                    req.page = Some(state.next_page.get());

                    let res = endpoints::jig::Search::api_no_auth(JigSearchPath(), Some(req))
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);

                    (
                        res.jigs.into_iter().map(Asset::from).collect(),
                        res.total_jig_count,
                    )
                },
                FeaturedKind::Playlists => {
                    let mut req = state.search_bar.search_selected.to_playlist_search_request();
                    state.active_query.set(req.q.clone());
                    req.page = Some(state.next_page.get());

                    let res = endpoints::playlist::Search::api_no_auth(PlaylistSearchPath(), Some(req))
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);

                    (
                        res.playlists.into_iter().map(Asset::from).collect(),
                        res.total_playlist_count,
                    )
                },
            };

            state.search_results
                .lock_mut()
                .extend(assets.into_iter().map(Rc::new));
            state.total_count.set(total as u32);
            state.next_page.replace_with(|next_page| *next_page + 1);
        }));
    }

    pub(super) fn load_featured(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            let assets: Vec<Asset> = match state.kind {
                FeaturedKind::Jigs => {
                    let res = endpoints::jig::Featured::api_with_auth(JigFeaturedPath(), None)
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);

                    res.jigs.into_iter().map(Asset::from).collect()
                },
                FeaturedKind::Playlists => {
                    let res = endpoints::playlist::Featured::api_with_auth(PlaylistFeaturedPath(), None)
                        .await
                        .toast_on_err();
                    let res = bail_on_err!(res);

                    res.playlists.into_iter().map(Asset::from).collect()
                },
            };

            state.featured_assets.lock_mut().extend(assets);
        }));
    }

    pub(super) fn update_featured(self: &Rc<Self>) {
        let state = self;
        spawn_local(clone!(state => async move {
            match state.kind {
                FeaturedKind::Jigs => {
                    let jigs: Option<Vec<JigId>> = state.featured_assets
                        .lock_ref()
                        .iter()
                        .map(|asset| match asset {
                            Asset::Jig(jig) => Some(jig.id),
                            _ => None,
                        })
                        .collect();
                    let Some(jigs) = jigs else {
                        toasts::error(state.kind.invalid_error());
                        return;
                    };

                    let req = JigFeaturedUpdateRequest { jigs };
                    let _ = endpoints::jig::FeaturedUpdate::api_with_auth(JigFeaturedPath(), Some(req))
                        .await
                        .toast_on_err();
                },
                FeaturedKind::Playlists => {
                    let playlists: Option<Vec<PlaylistId>> = state.featured_assets
                        .lock_ref()
                        .iter()
                        .map(|asset| match asset {
                            Asset::Playlist(playlist) => Some(playlist.id),
                            _ => None,
                        })
                        .collect();
                    let Some(playlists) = playlists else {
                        toasts::error(state.kind.invalid_error());
                        return;
                    };

                    let req = PlaylistFeaturedUpdateRequest { playlists };
                    let _ = endpoints::playlist::FeaturedUpdate::api_with_auth(PlaylistFeaturedPath(), Some(req))
                        .await
                        .toast_on_err();
                },
            }
        }));
    }
}
