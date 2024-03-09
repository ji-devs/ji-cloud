use std::rc::Rc;

use dominator::clone;
use futures::join;
use shared::domain::playlist::{PlaylistGetDraftPath, PlaylistId};
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        meta::GetMetadataPath,
        playlist::{
            PlaylistBrowsePath, PlaylistBrowseQuery, PlaylistResponse, PlaylistSearchPath,
            PlaylistSearchQuery,
        },
    },
};
use utils::{
    editable_asset::EditablePlaylist,
    prelude::ApiEndpointExt,
    routes::{AdminPlaylistCurationRoute, AdminRoute, Route},
    unwrap::UnwrapJiExt,
};

use super::{FetchMode, PlaylistCuration};

impl PlaylistCuration {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.loader.load(clone!(state => async move {
            join!(
                state.load_playlists(),
                state.load_meta()
            );
        }));
    }

    async fn load_meta(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.ages.set(meta.age_ranges);
                self.affiliations.set(meta.affiliations);
            }
        };
    }

    pub async fn load_playlists(self: &Rc<Self>) {
        // clone right away to free the lock
        let fetch_mode = self.fetch_mode.borrow().clone();
        let res = match fetch_mode {
            FetchMode::Browse => self.load_playlists_browse().await,
            FetchMode::Search(query) => self.load_playlists_search(query.clone()).await,
        };

        self.playlists.lock_mut().replace_cloned(
            res.playlists
                .into_iter()
                .map(|playlist| Rc::new(playlist.into()))
                .collect(),
        );
        // self.set_total_page(res.total_page);

        self.total_pages.set_neq(Some(res.total_pages));
    }

    async fn load_playlists_browse(&self) -> PlaylistListResponse {
        let req = PlaylistBrowseQuery {
            page: Some(self.active_page.get()),
            draft_or_live: Some(DraftOrLive::Live),
            ..Default::default()
        };

        match endpoints::playlist::Browse::api_with_auth(PlaylistBrowsePath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => PlaylistListResponse {
                playlists: res.playlists,
                total_pages: res.pages,
            },
        }
    }

    async fn load_playlists_search(&self, query: String) -> PlaylistListResponse {
        let req = PlaylistSearchQuery {
            q: query,
            page: Some(self.active_page.get()),
            ..Default::default()
        };

        match endpoints::playlist::Search::api_with_auth(PlaylistSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => PlaylistListResponse {
                playlists: res.playlists,
                total_pages: res.pages,
            },
        }
    }

    pub fn go_to_page(self: &Rc<Self>, page: u32) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.active_page.set(page);
            state.load_playlists().await;
        }));
    }

    pub fn navigate_to(self: &Rc<Self>, route: AdminPlaylistCurationRoute) {
        self.route.set(route.clone());
        Route::Admin(AdminRoute::PlaylistCuration(route)).push_state();
    }

    pub async fn get_playlist(self: Rc<Self>, playlist_id: PlaylistId) -> Rc<EditablePlaylist> {
        let jig = self
            .playlists
            .lock_ref()
            .iter()
            .find(|playlist| playlist.id == playlist_id)
            .cloned();
        match jig {
            Some(playlist) => playlist,
            None => Rc::new(self.load_playlist(&playlist_id).await),
        }
    }

    async fn load_playlist(self: &Rc<Self>, playlist_id: &PlaylistId) -> EditablePlaylist {
        match endpoints::playlist::GetDraft::api_with_auth(
            PlaylistGetDraftPath(playlist_id.clone()),
            None,
        )
        .await
        {
            Ok(jig) => jig.into(),
            Err(_) => {
                todo!()
            }
        }
    }

    pub fn save_and_publish(self: &Rc<Self>, playlist: &Rc<EditablePlaylist>) {
        self.loader.load(clone!(playlist => async move {
            let (a, b) = join!(
                playlist.save_draft(),
                playlist.save_admin_data(),
            );
            a.unwrap_ji();
            b.unwrap_ji();
            playlist.publish().await.unwrap_ji();
        }))
    }

    pub fn save_admin_data(self: &Rc<Self>, playlist: &Rc<EditablePlaylist>) {
        self.loader.load(clone!(playlist => async move {
            playlist.save_admin_data().await.unwrap_ji();
        }))
    }
}

#[derive(Clone, Debug)]
struct PlaylistListResponse {
    playlists: Vec<PlaylistResponse>,
    total_pages: u32,
}
