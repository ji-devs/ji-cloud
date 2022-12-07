use std::rc::Rc;

use components::share_asset::ShareAsset;
use dominator_helpers::futures::AsyncLoader;

use super::super::super::state::AssetEditState;

pub struct PostPublish {
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareAsset>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl PostPublish {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            // share_state: ShareAsset::new(asset.clone()),
            share_state: ShareAsset::new(todo!()),
            loader: AsyncLoader::new(),
            asset_edit_state,
        })
    }
}
