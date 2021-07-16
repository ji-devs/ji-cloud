//! EventArc

use crate::error;
use core::settings::GoogleCloudEventArcSettings;

pub struct Client {
    oauth2_token: String,
    media_uploaded_topic: String,
    media_processed_topic: String,
}

impl Client {
    pub fn new(settings: GoogleCloudEventArcSettings) -> anyhow::Result<Self> {
        let GoogleCloudEventArcSettings {
            oauth2_token,
            media_uploaded_topic,
            media_processed_topic,
        } = settings;

        Ok(Self {
            oauth2_token,
            media_uploaded_topic,
            media_processed_topic,
        })
    }
}
