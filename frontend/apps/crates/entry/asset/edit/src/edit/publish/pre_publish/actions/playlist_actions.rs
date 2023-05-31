use shared::{
    api::endpoints::playlist,
    domain::playlist::{
        PlaylistGetDraftPath, PlaylistId, PlaylistPublishPath, PlaylistResponse,
        PlaylistUpdateDraftDataPath,
    },
};
use utils::prelude::ApiEndpointExt;

use utils::editable_asset::EditablePlaylist;

pub async fn save_playlist(playlist: &EditablePlaylist) -> anyhow::Result<()> {
    let req = playlist.to_playlist_update_request();

    playlist::UpdateDraftData::api_with_auth_empty(
        PlaylistUpdateDraftDataPath(playlist.id),
        Some(req),
    )
    .await
}

pub async fn publish_playlist(playlist_id: PlaylistId) -> anyhow::Result<PlaylistResponse> {
    playlist::Publish::api_with_auth_empty(PlaylistPublishPath(playlist_id), None).await?;

    let playlist =
        playlist::GetDraft::api_with_auth(PlaylistGetDraftPath(playlist_id), None).await?;

    Ok(playlist)
}
