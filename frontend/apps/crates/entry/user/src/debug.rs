use cfg_if::cfg_if;
use once_cell::sync::OnceCell;
use utils::prelude::*;

pub static SETTINGS:OnceCell<DebugSettings> = OnceCell::new();


#[derive(Debug, Default)]
pub struct DebugSettings {
    pub skip_password_strength: bool,
}


impl DebugSettings {
    pub fn debug() -> DebugSettings {
        DebugSettings {
            skip_password_strength: true
        }
    }
}

cfg_if! {
    if #[cfg(debug_assertions)] {
        pub fn init() {
            SETTINGS.set(DebugSettings::debug()).unwrap_ji();
        }
    } else {
        pub fn init() {
            SETTINGS.set(DebugSettings::default()).unwrap_ji();
        }
    }
}


pub fn settings() -> &'static DebugSettings {
    unsafe { SETTINGS.get_unchecked() }
}
