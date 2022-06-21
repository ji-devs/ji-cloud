use std::rc::Rc;

use components::share_asset::ShareAsset;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::asset::AssetId;

use super::super::state::AssetEditState;

pub struct PostPublish {
    pub asset_id: AssetId,
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareAsset>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl PostPublish {
    pub fn new(asset_id: AssetId, asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            loader: AsyncLoader::new(),
            share_state: ShareAsset::new(asset_id),
            asset_edit_state,
        })
    }
}
