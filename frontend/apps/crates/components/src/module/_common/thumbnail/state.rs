use std::rc::Rc;

use shared::domain::{
    asset::{AssetId, DraftOrLive},
    module::LiteModule,
};

pub struct ModuleThumbnail {
    pub asset_id: AssetId,
    pub module: Option<LiteModule>,
    pub fallback: ThumbnailFallback,
    pub draft_or_live: DraftOrLive,
}

impl ModuleThumbnail {
    pub fn new(
        asset_id: AssetId,
        module: Option<LiteModule>,
        fallback: ThumbnailFallback,
        draft_or_live: DraftOrLive,
    ) -> Rc<Self> {
        Rc::new(ModuleThumbnail {
            asset_id,
            module,
            fallback,
            draft_or_live,
        })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThumbnailFallback {
    Asset,
    Module,
}
