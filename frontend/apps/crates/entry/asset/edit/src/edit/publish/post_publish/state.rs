use std::rc::Rc;

use components::share_asset::ShareAsset;
use shared::domain::asset::Asset;

use super::super::super::state::AssetEditState;

pub struct PostPublish {
    pub share_state: Rc<ShareAsset>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl PostPublish {
    pub fn new(asset_edit_state: Rc<AssetEditState>, asset: &Asset) -> Rc<Self> {
        Rc::new(Self {
            share_state: ShareAsset::new(asset.clone()),
            asset_edit_state,
        })
    }
}
