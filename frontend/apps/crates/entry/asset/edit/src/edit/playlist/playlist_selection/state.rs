use std::rc::Rc;

use components::asset_search_bar::AssetSearchBar;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    asset::{Asset, AssetId},
    jig::JigResponse,
    playlist::PlaylistId,
};
use utils::{asset::AssetPlayerOptions, drag::Drag};

use crate::edit::AssetEditState;

pub struct PlaylistSelection {
    pub playlist_id: PlaylistId,
    pub search_bar: Rc<AssetSearchBar>,
    pub asset_edit_state: Rc<AssetEditState>,
    pub loader: AsyncLoader,
    pub search_results: MutableVec<Rc<JigResponse>>,
    pub next_page: Mutable<u32>,
    pub active_query: Mutable<String>,
    pub total_jig_count: Mutable<u32>,
    pub drag: Mutable<Option<Rc<Drag<Asset>>>>,
    pub(super) play_asset: Mutable<Option<(AssetId, AssetPlayerOptions)>>,
}

impl PlaylistSelection {
    pub fn new(playlist_id: PlaylistId, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            playlist_id,
            search_bar: AssetSearchBar::new(),
            asset_edit_state: Rc::clone(asset_edit_state),
            loader: AsyncLoader::new(),
            search_results: Default::default(),
            next_page: Default::default(),
            active_query: Default::default(),
            total_jig_count: Default::default(),
            drag: Default::default(),
            play_asset: Default::default(),
        })
    }
}
