use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::AssetId,
    module::{ModuleId, ModuleKind},
};
use std::rc::Rc;

pub struct ModuleIframe {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub module_kind: Mutable<Option<ModuleKind>>,
    pub loader: AsyncLoader,
}

impl ModuleIframe {
    pub fn new(asset_id: AssetId, module_id: ModuleId) -> Rc<Self> {
        Rc::new(Self {
            asset_id,
            module_id,
            module_kind: Default::default(),
            loader: AsyncLoader::new(),
        })
    }
}
