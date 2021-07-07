//! EventArc

use crate::error;
use core::settings::GoogleCloudEventArcSettings;

pub struct Client {
    oauth2_token: String,
    media_uploaded_trigger: String,
    media_processed_trigger: String,
}

impl Client {
    pub fn new(settings: GoogleCloudEventArcSettings) -> anyhow::Result<Self> {
        let GoogleCloudEventArcSettings {
            oauth2_token,
            media_uploaded_trigger,
            media_processed_trigger,
        } = settings;

        Ok(Self {
            oauth2_token,
            media_uploaded_trigger,
            media_processed_trigger,
        })
    }
}
