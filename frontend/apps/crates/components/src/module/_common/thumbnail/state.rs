use shared::domain::{asset::AssetId, jig::module::LiteModule};

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
