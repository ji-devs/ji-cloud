use shared::domain::jig::{JigId, module::{LiteModule}};

pub struct ModuleThumbnail {
    pub jig_id: JigId,
    pub module: LiteModule,
    /// If this is set to true it will use the jig thumbnail fallback
    /// Otherwise it uses the module thumbnail fallback
    pub is_jig_fallback: bool,
}
