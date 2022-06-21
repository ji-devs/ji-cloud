use std::rc::Rc;

use components::share_jig::ShareJig;
use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::JigId;

use super::super::state::AssetEditState;

pub struct PostPublish {
    pub jig_id: JigId,
    pub loader: AsyncLoader,
    pub share_state: Rc<ShareJig>,
    pub asset_edit_state: Rc<AssetEditState>,
}

impl PostPublish {
    pub fn new(jig_id: JigId, asset_edit_state: Rc<AssetEditState>) -> Rc<Self> {
        Rc::new(Self {
            jig_id,
            loader: AsyncLoader::new(),
            share_state: ShareJig::new(jig_id),
            asset_edit_state,
        })
    }
}
