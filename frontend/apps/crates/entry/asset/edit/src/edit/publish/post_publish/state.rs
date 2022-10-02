use std::rc::Rc;

use components::share_asset::ShareAsset;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::asset::Asset;

use super::super::super::state::AssetEditState;

pub struct PostPublish {
    pub asset: Asset,
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareAsset>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl PostPublish {
    pub fn new(asset: Asset, asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            share_state: ShareAsset::new(asset.clone()),
            asset,
            loader: AsyncLoader::new(),
            asset_edit_state,
        })
    }
}
