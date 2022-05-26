use std::rc::Rc;

use shared::domain::{asset::AssetId, module::LiteModule};

pub struct ModuleThumbnail {
    pub asset_id: AssetId,
    pub module: Option<LiteModule>,
    pub fallback: ThumbnailFallback,
}

impl ModuleThumbnail {
    pub fn new(
        asset_id: AssetId,
        module: Option<LiteModule>,
        fallback: ThumbnailFallback,
    ) -> Rc<Self> {
        Rc::new(ModuleThumbnail {
            asset_id,
            module,
            fallback,
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThumbnailFallback {
    Asset,
    Module,
}
