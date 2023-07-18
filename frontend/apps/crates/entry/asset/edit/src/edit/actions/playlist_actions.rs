use shared::{
    api::endpoints::{self, playlist},
    domain::{
        jig::{JigGetLivePath, JigId, JigResponse},
        playlist::{PlaylistGetDraftPath, PlaylistId, PlaylistResponse},
    },
};
use utils::prelude::{ApiEndpointExt, ErrorExt, UnwrapJiExt};

use crate::edit::{sidebar::SidebarSpot, AssetEditState};

pub async fn load_playlist(playlist_id: PlaylistId) -> anyhow::Result<PlaylistResponse> {
    playlist::GetDraft::api_with_auth(PlaylistGetDraftPath(playlist_id.clone()), None)
        .await
        .into_anyhow()
}

impl AssetEditState {
    pub async fn get_playlist_spots(&self, playlist: &PlaylistResponse) {
        let mut items = vec![SidebarSpot::new_playlist_cover(
            playlist.playlist_data.cover.clone().unwrap_ji(),
        )];
        for jig_id in &playlist.playlist_data.items {
            let jig = get_jig(jig_id).await;

            items.push(SidebarSpot::new_playlist_item(jig));
        }

        // add empty at the end
        items.push(SidebarSpot::new_empty(&playlist.id.into(), None));

        let mut spots = self.sidebar_spots.lock_mut();
        for item in items {
            spots.push_cloned(item);
        }
    }
}

async fn get_jig(jig_id: &JigId) -> JigResponse {
    endpoints::jig::GetLive::api_with_auth(JigGetLivePath(jig_id.clone()), None)
        .await
        .unwrap_ji()
}
