use cfg_if::cfg_if;

#[derive(Default)]
pub struct DebugSettings {}

impl DebugSettings {
    pub fn local() -> Self {
        Self {}
    }
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    } else {
        pub fn settings() -> DebugSettings {
            DebugSettings::default()
        }
    }
}
