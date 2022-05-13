use shared::domain::{jig::module::LiteModule, asset::AssetId};

pub struct ModuleThumbnail {
    pub asset_id: AssetId,
    pub module: Option<LiteModule>,
    pub fallback: ThumbnailFallback,
}


#[derive(Clone, Copy, PartialEq)]
pub enum ThumbnailFallback {
    Asset,
    Module,
}
