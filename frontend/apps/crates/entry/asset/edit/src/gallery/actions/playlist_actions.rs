use super::super::state::Gallery;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
        playlist::{
            PlaylistBrowsePath, PlaylistBrowseQuery, PlaylistClonePath, PlaylistCreatePath,
            PlaylistCreateRequest, PlaylistDeletePath, PlaylistGetDraftPath, PlaylistId,
            PlaylistSearchPath, PlaylistSearchQuery,
        },
    },
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

pub async fn create_playlist() {
    let req = PlaylistCreateRequest {
        ..Default::default()
    };

    match endpoints::playlist::Create::api_with_auth(PlaylistCreatePath(), Some(req)).await {
        Ok(resp) => {
            add_cover(&resp.id).await;
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                resp.id,
                PlaylistEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

async fn add_cover(playlist_id: &PlaylistId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*playlist_id).into(),
    };

    // let path = endpoints::module::Create::PATH.replace("{id}", &jig_id.0.to_string());

    match endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await {
        Ok(_) => {}
        Err(_) => {
            todo!()
        }
    }
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
    endpoints::playlist::Delete::api_with_auth_empty(PlaylistDeletePath(playlist_id), None)
        .await
        .map(|_| ())
}
