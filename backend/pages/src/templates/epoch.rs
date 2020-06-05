use crate::settings::SETTINGS;

pub fn epoch_page() -> String {
    format!("{}", SETTINGS.epoch.as_millis())
}