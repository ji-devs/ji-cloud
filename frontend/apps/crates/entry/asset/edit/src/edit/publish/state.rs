use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::asset::Asset;

use super::super::state::AssetEditState;

pub struct Publish {
    // None until after publish
    pub published_asset: Mutable<Option<Asset>>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl Publish {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            published_asset: Mutable::new(None),
            asset_edit_state,
        })
    }
}
