use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;

use super::super::state::AssetEditState;

pub struct Publish {
    // pub asset_id: AssetId,
    pub asset: Mutable<Option<Asset>>,
    pub post_publish: Mutable<bool>,
    pub loader: AsyncLoader,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl Publish {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            // asset_id,
            asset: Default::default(),
            loader: AsyncLoader::new(),
            post_publish: Mutable::new(false),
            asset_edit_state,
        })
    }
}
