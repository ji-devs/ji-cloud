use shared::domain::jig::{AudioEffects, Jig, JigId, LiteModule, ModuleKind, TextDirection, module::ModuleId};
use utils::themes::ThemeId;
use uuid::Uuid;
use cfg_if::cfg_if;

pub fn get_jig() -> Jig {
    let module_id = ModuleId(Uuid::from_u128(0));
    Jig {
        id: JigId(Uuid::from_u128(0)),
        display_name: "hello world".to_string(),
        //TODO - delete me: https://github.com/ji-devs/ji-cloud/issues/835
        modules: vec![
            LiteModule {
                id: module_id,
                kind: ModuleKind::Cover
            },
            LiteModule {
                id: module_id,
                kind: ModuleKind::Memory
            },
            LiteModule {
                id: module_id,
                kind: ModuleKind::Memory
            },
            LiteModule {
                id: module_id,
                kind: ModuleKind::TappingBoard
            },
        ],
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        goals: Vec::new(),
        creator_id: None,
        author_id: None,
        language: String::new(),
        categories: Vec::new(),
        publish_at: None,
        additional_resources: Vec::new(),
        description: String::new(),
        last_edited: None,
        is_public: false,
        direction: TextDirection::LeftToRight,
        display_score: true,
        theme: ThemeId::Chalkboard,
        audio_background: None,
        audio_effects: AudioEffects::default(),
    }

}
