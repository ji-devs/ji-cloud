use super::super::state::Gallery;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        playlist::{
            PlaylistBrowsePath, PlaylistBrowseQuery, PlaylistClonePath, PlaylistDeletePath,
            PlaylistGetDraftPath, PlaylistId, PlaylistSearchPath, PlaylistSearchQuery,
        },
    },
    error::IntoAnyhow,
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_playlists(
    state: &Rc<Gallery>,
    is_published: Option<bool>,
) -> Result<(Vec<Asset>, u64), ()> {
    let req = PlaylistBrowseQuery {
        page: Some(*state.next_page.lock_ref()),
        is_published,
        author_id: Some(UserOrMe::Me),
        draft_or_live: Some(DraftOrLive::Draft),
        ..Default::default()
    };

    endpoints::playlist::Browse::api_with_auth(PlaylistBrowsePath(), Some(req))
        .await
        .map(|res| {
            let assets = res
                .playlists
                .into_iter()
                .map(|playlist| playlist.into())
                .collect();
            (assets, res.total_playlist_count)
        })
        .map_err(|_| ())
}

pub async fn search_playlists(q: String, is_published: Option<bool>) -> Result<Vec<Asset>, ()> {
    let req = PlaylistSearchQuery {
        q,
        is_published,
        author_id: Some(UserOrMe::Me),
        ..Default::default()
    };

    endpoints::playlist::Search::api_with_auth(PlaylistSearchPath(), Some(req))
        .await
        .map(|resp| {
            resp.playlists
                .into_iter()
                .map(|playlist| playlist.into())
                .collect()
        })
        .map_err(|_| ())
}

pub async fn copy_playlist(playlist_id: PlaylistId) -> Result<Asset, ()> {
    match endpoints::playlist::Clone::api_with_auth(PlaylistClonePath(playlist_id), None).await {
        Ok(resp) => {
            endpoints::playlist::GetDraft::api_with_auth(PlaylistGetDraftPath(resp.id), None)
                .await
                .map(|resp| {
                    let asset: Asset = resp.into();
                    asset
                })
                .map_err(|_| ())
        }
        Err(_) => Err(()),
    }
}

pub async fn delete_playlist(playlist_id: PlaylistId) -> anyhow::Result<()> {
    endpoints::playlist::Delete::api_with_auth(PlaylistDeletePath(playlist_id), None)
        .await
        .into_anyhow()
}
