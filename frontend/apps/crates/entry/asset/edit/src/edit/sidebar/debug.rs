#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

use shared::domain::{
    asset::{DraftOrLive, PrivacyLevel},
    jig::{AudioEffects, JigAdminData, JigData, JigId, JigPlayerSettings, JigRating, JigResponse},
    module::{LiteModule, ModuleId, ModuleKind},
};
use utils::themes::ThemeId;
use uuid::Uuid;

pub fn get_jig() -> JigResponse {
    let module_id = ModuleId::from_u128(0);
    JigResponse {
        id: JigId::from_u128(0),
        admin_data: JigAdminData {
            rating: None,
            blocked: false,
            curated: true,
            premium: false,
        },
        creator_id: None,
        author_id: None,
        author_name: None,
        published_at: None,
        live_up_to_date: false,
        is_liked: false,
        jig_data: JigData {
            created_at: chrono::offset::Utc::now(),
            draft_or_live: DraftOrLive::Draft,
            display_name: "hello world".to_string(),
            //TODO - delete me: https://github.com/ji-devs/ji-cloud/issues/835
            modules: vec![
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Cover,
                    is_complete: true,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Memory,
                    is_complete: true,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::Memory,
                    is_complete: true,
                },
                LiteModule {
                    id: module_id,
                    kind: ModuleKind::TappingBoard,
                    is_complete: true,
                },
            ],
            age_ranges: Vec::new(),
            affiliations: Vec::new(),
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
            locked: true,
            other_keywords: String::from(""),
            translated_keywords: String::from(""),
            translated_description: HashMap::new(),
        },
        likes: 0,
        plays: 0,
    }
}
