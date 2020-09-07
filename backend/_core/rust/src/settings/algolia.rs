use crate::env::{env_bool, keys};
use config::RemoteTarget;

pub struct AlgoliaSettings {
    pub application_id: String,
    pub key: String,
}

impl AlgoliaSettings {
    pub(crate) fn new(application_id: String, key: String) -> Option<Self> {
        let disable_local = env_bool(keys::algolia::DISABLE);
        if matches!(crate::REMOTE_TARGET, RemoteTarget::Local) && disable_local {
            return None;
        }

        Some(AlgoliaSettings {
            application_id,
            key,
        })
    }
}
