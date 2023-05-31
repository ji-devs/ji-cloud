use std::rc::Rc;

use dominator::clone;
use futures::future::try_join_all;
use futures::join;
use shared::{
    api::endpoints,
    domain::{
        asset::DraftOrLive,
        jig::{JigGetLivePath, JigId, JigResponse},
        meta::GetMetadataPath,
        playlist::{PlaylistGetDraftPath, PlaylistGetLivePath},
    },
};
use utils::{
    iframe::{AssetPlayerToPlayerPopup, IframeAction, IframeMessageExt},
    js_wrappers::is_iframe,
    prelude::ApiEndpointExt,
    unwrap::UnwrapJiExt,
};

use super::state::PlaylistPlayer;

impl PlaylistPlayer {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            join!(
                state.load_playlist(),
                state.load_resource_types(),
            );
        }));
    }

    async fn load_playlist(self: &Rc<Self>) {
        let state = self;
        let playlist = match state.player_options.draft_or_live {
            DraftOrLive::Live => {
                endpoints::playlist::GetLive::api_no_auth(
                    PlaylistGetLivePath(state.playlist_id),
                    None,
                )
                .await
            }
            DraftOrLive::Draft => {
                endpoints::playlist::GetDraft::api_no_auth(
                    PlaylistGetDraftPath(state.playlist_id),
                    None,
                )
                .await
            }
        };

        match playlist {
            Ok(playlist) => {
                let jig_ids = playlist.playlist_data.items.clone();
                state.playlist.set(Some(playlist));
                state.load_jigs(jig_ids).await;
            }
            Err(_) => {
                todo!();
            }
        }
    }

    async fn load_resource_types(self: &Rc<Self>) {
        match endpoints::meta::Get::api_with_auth(GetMetadataPath(), None).await {
            Err(_) => todo!(),
            Ok(meta) => {
                self.resource_types.set(meta.resource_types);
            }
        };
    }

    async fn load_jigs(self: &Rc<Self>, jig_ids: Vec<JigId>) {
        let jigs = try_join_all(jig_ids.iter().map(|jig_id| self.load_jig(jig_id)))
            .await
            .unwrap_ji();

        self.jigs.set(jigs);
    }

    async fn load_jig(self: &Rc<Self>, jig_id: &JigId) -> Result<JigResponse, ()> {
        endpoints::jig::GetLive::api_no_auth(JigGetLivePath(jig_id.clone()), None)
            .await
            .map_err(|_| ())
    }

    pub fn play_jig(self: &Rc<Self>, jig_id: JigId) {
        self.active_jig.set(Some(jig_id));
        if is_iframe() {
            let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(false))
                .try_post_message_to_parent();
        }
    }

    pub fn done_playing_jig(self: &Rc<Self>) {
        self.active_jig.set(None);
        if is_iframe() {
            let _ = IframeAction::new(AssetPlayerToPlayerPopup::CloseButtonShown(true))
                .try_post_message_to_parent();
        }
    }
}
