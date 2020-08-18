use crate::env::{env_bool, req_env};
use config::RemoteTarget;

pub struct AlgoliaSettings {
    pub application_id: String,
    pub key: String,
}

impl AlgoliaSettings {
    pub fn new() -> anyhow::Result<Option<Self>> {
        let disable_local = env_bool("ALGOLIA_LOCAL_DISABLE_CLIENT");
        if matches!(crate::REMOTE_TARGET, RemoteTarget::Local) && disable_local {
            return Ok(None);
        }

        Ok(Some(AlgoliaSettings {
            application_id: req_env("ALGOLIA_APPLICATION_ID")?,
            key: req_env("ALGOLIA_KEY")?,
        }))
    }
}
