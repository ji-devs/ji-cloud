use dominator_helpers::futures::AsyncLoader;
use shared::domain::{
    asset::AssetId,
    module::{ModuleId, ModuleKind},
};

pub struct PostPreview {
    pub module_kind: ModuleKind,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub loader: AsyncLoader,
}

impl PostPreview {
    pub fn new(module_kind: ModuleKind, asset_id: AssetId, module_id: ModuleId) -> Self {
        Self {
            module_kind,
            asset_id,
            module_id,
            loader: AsyncLoader::new(),
        }
    }
}
