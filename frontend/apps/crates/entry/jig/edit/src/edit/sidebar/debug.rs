use shared::domain::jig::{Jig, JigId, LiteModule, module::ModuleId, ModuleKind};
use uuid::Uuid;
use cfg_if::cfg_if;

pub fn get_jig() -> Jig {
    let module_id = ModuleId(Uuid::from_u128(0));
    Jig {
        id: JigId(Uuid::from_u128(0)),
        display_name: Some("hello world".to_string()),
        //TODO - delete me: https://github.com/ji-devs/ji-cloud/issues/835
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
        goals: Vec::new(),
        creator_id: None,
        author_id: None,
        publish_at: None,
        affiliations: Vec::new(),
        age_ranges: Vec::new(),
        categories: Vec::new(),
        language: String::new(),
    }

}
