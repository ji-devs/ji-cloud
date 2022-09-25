use cfg_if::cfg_if;

#[derive(Default)]
pub struct DebugSettings {
    pub empty_module_url: &'static str,
}

#[cfg(feature = "local")]
impl DebugSettings {
    pub fn local() -> Self {
        Self {
            empty_module_url: "http://localhost:4105/module/legacy/play/debug",
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
