use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;

use super::super::state::AssetEditState;

pub struct Publish {
    // pub asset_id: AssetId,
    pub post_publish: Mutable<bool>,
    pub loader: AsyncLoader,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl Publish {
    pub fn new(asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            // asset_id,
            loader: AsyncLoader::new(),
            post_publish: Mutable::new(false),
            asset_edit_state,
        })
    }
}
