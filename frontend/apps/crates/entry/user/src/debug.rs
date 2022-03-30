#![allow(dead_code)]
use cfg_if::cfg_if;
use once_cell::sync::OnceCell;
use utils::prelude::*;

pub static SETTINGS: OnceCell<DebugSettings> = OnceCell::new();

#[derive(Debug, Default)]
pub struct DebugSettings {
    pub register_step: Option<DebugRegisterStep>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugRegisterStep {
    Start,
    One,
    Two,
    Three,
}

impl DebugSettings {
    pub fn debug() -> DebugSettings {
        DebugSettings {
            //register_step: Some(DebugRegisterStep::Two),
            register_step: None,
        }
    }
}

cfg_if! {
    if #[cfg(all(feature = "local", debug_assertions))] {
        pub fn init() {
            SETTINGS.set(DebugSettings::debug()).unwrap_ji();
            //SETTINGS.set(DebugSettings::default()).unwrap_ji();
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
