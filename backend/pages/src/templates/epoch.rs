use core::settings::SETTINGS;

pub fn epoch_page() -> String {
    format!("{}", SETTINGS.get().unwrap().epoch.as_millis())
}
