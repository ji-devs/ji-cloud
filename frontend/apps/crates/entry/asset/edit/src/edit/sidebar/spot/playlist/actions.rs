use super::super::super::spot::state::SpotState;
use crate::edit::sidebar::{PlaylistSpot, SidebarSpot, SidebarSpotItem};
use itertools::Itertools;
use shared::{
    api::endpoints,
    domain::{
        asset::Asset,
        playlist::{PlaylistUpdateDraftDataPath, PlaylistUpdateDraftDataRequest},
    },
};
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;

pub async fn save_playlist(state: &Rc<SpotState>) {
    let items = state
        .sidebar
        .asset_edit_state
        .sidebar_spots
        .lock_ref()
        .iter()
        .filter_map(|spot| {
            // filter out cover and empty spots
            match &spot.item {
                SidebarSpotItem::Jig(_) => unreachable!(),
                SidebarSpotItem::Playlist(spot) => match spot {
                    None => None,
                    Some(spot) => match &**spot {
                        PlaylistSpot::Cover(_) => None,
                        PlaylistSpot::Item(jig) => Some(jig.id),
                    },
                },
                SidebarSpotItem::Course(_) => unreachable!(),
            }
        })
        .collect_vec();

    let req = PlaylistUpdateDraftDataRequest {
        items: Some(items),
        ..Default::default()
    };

    let _ = endpoints::playlist::UpdateDraftData::api_with_auth(
        PlaylistUpdateDraftDataPath(*state.sidebar.asset_edit_state.asset_id.unwrap_playlist()),
        Some(req),
    )
    .await;
}

// pub fn assign_asset_to_empty_spot(_state: &Rc<SpotState>, asset: Asset) -> Rc<SidebarSpot> {
//     let jig = match asset {
//         Asset::Jig(jig) => jig,
//         Asset::Resource(_) => todo!(),
//         Asset::Playlist(_) => unreachable!(),
//     };
//     SidebarSpot::new_playlist_item(jig)
// }

pub async fn assign_asset_to_empty_spot(_state: &Rc<SpotState>, asset: Asset) -> Rc<SidebarSpot> {
    let jig = match asset {
        Asset::Jig(jig) => jig,
        Asset::Resource(_) => todo!(),
        Asset::Playlist(_) => unreachable!(),
        Asset::Course(_) => todo!(),
    };
    SidebarSpot::new_playlist_item(jig)
}
