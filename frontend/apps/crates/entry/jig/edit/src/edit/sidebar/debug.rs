#![allow(unused_imports)]
#![allow(dead_code)]

use shared::domain::jig::{
    AudioEffects,
    DraftOrLive,
    JigAdminData,
    JigData,
    JigFocus,
    JigId,
    JigPlayerSettings,
    JigRating,
    JigResponse,
    LiteModule,
    ModuleKind,
    PrivacyLevel,
    module::ModuleId
};
use utils::themes::ThemeId;
use uuid::Uuid;

pub fn get_jig() -> JigResponse {
    let module_id = ModuleId(Uuid::from_u128(0));
    JigResponse {
        id: JigId(Uuid::from_u128(0)),
        admin_data: JigAdminData {
            rating: Some(JigRating::NoRating),
            blocked: false,
            curated: true,
        },
        creator_id: None,
        author_id: None,
        author_name: None,
        published_at: None,
        first_cover_assigned: true,
        jig_data: JigData {
            draft_or_live: DraftOrLive::Draft,
            display_name: "hello world".to_string(),
            //TODO - delete me: https://github.com/ji-devs/ji-cloud/issues/835
            modules: vec![
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Cover,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Memory,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Memory,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::TappingBoard,
                },
            ],
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
            goals: Vec::new(),
            language: String::new(),
            categories: Vec::new(),
            additional_resources: Vec::new(),
            description: String::new(),
            last_edited: None,
            theme: ThemeId::Chalkboard,
            audio_background: None,
            audio_effects: AudioEffects::default(),
            default_player_settings: JigPlayerSettings::default(),
            privacy_level: PrivacyLevel::default(),
            jig_focus: JigFocus::Modules,
            locked: true,
            other_keywords: String::from(""),
            translated_keywords: String::from(""),
        },
        likes: 0,
        plays: 0,
    }
}
