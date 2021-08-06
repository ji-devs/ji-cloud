use dominator_helpers::futures::AsyncLoader;
use shared::domain::jig::{module::ModuleId, JigId, ModuleKind};

pub struct PostPreview {
    pub module_kind: ModuleKind,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub loader: AsyncLoader,
}

impl PostPreview {
    pub fn new(module_kind: ModuleKind, jig_id: JigId, module_id: ModuleId) -> Self {
        Self {
            module_kind,
            jig_id,
            module_id,
            loader: AsyncLoader::new(),
        }
    }
}
