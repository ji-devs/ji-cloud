use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;

use super::super::state::AssetEditState;

pub struct Publish {
    // None until after publish
    pub published_asset: Mutable<Option<Asset>>,
    pub loader: AsyncLoader,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl Publish {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            published_asset: Mutable::new(None),
            loader: AsyncLoader::new(),
            asset_edit_state,
        })
    }
}
