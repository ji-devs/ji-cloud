use crate::data::*;
use cfg_if::cfg_if;

#[derive(Default)]
pub struct DebugSettings {
    pub game_mode:Option<Option<GameMode>>,
    pub step:Option<Step>
}

impl DebugSettings {
    pub fn local() -> Self {
        Self {
            game_mode: Some(Some(GameMode::Duplicate)),
            step: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::local()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}

