use shared::domain::jig::{Jig, JigId, LiteModule, ModuleId, ModuleKind};
use uuid::Uuid;
use cfg_if::cfg_if;

pub fn get_jig() -> Jig {
    let module_id = ModuleId(Uuid::from_u128(0));
    Jig {
        id: JigId(Uuid::from_u128(0)),
        display_name: Some("hello world".to_string()),
        //TODO - delete me: https://github.com/ji-devs/ji-cloud/issues/835
        cover: LiteModule {
            id: module_id,
            kind: Some(ModuleKind::Cover)
        },
        ending: LiteModule {
            id: module_id,
            kind: Some(ModuleKind::Cover)
        },
        modules: vec![
            LiteModule {
                id: module_id,
                kind: Some(ModuleKind::Cover)
            },
            LiteModule {
                id: module_id,
                kind: Some(ModuleKind::Memory)
            },
            LiteModule {
                id: module_id,
                kind: None 
            },
            LiteModule {
                id: module_id,
                kind: Some(ModuleKind::TappingBoard)
            },
        ],
        content_types: Vec::new(),
        creator_id: None,
        author_id: None,
        publish_at: None,
    }

}
