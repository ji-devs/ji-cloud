use crate::env::{env_bool, req_env};
use config::RemoteTarget;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RuntimeSettings {
    firebase_no_auth: bool,
    pub api_port: u16,
    pub pages_port: u16,
    pub epoch: Duration,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    jwt_decoding_key: String,
}

impl RuntimeSettings {
    pub(crate) fn new(
        jwt_encoding_key: EncodingKey,
        jwt_decoding_key: String,
    ) -> anyhow::Result<Self> {
        let (api_port, pages_port) = match crate::REMOTE_TARGET {
            RemoteTarget::Local => (
                req_env("LOCAL_API_PORT")?.parse()?,
                req_env("LOCAL_PAGES_PORT")?.parse()?,
            ),

            RemoteTarget::Sandbox | RemoteTarget::Release => (8080_u16, 8080_u16),
        };

        let firebase_no_auth = env_bool("LOCAL_NO_FIREBASE_AUTH");

        Ok(Self {
            api_port,
            pages_port,
            firebase_no_auth,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
        })
    }

    // shh, we're pretending not to be pulling this out of the aether
    pub fn remote_target(&self) -> RemoteTarget {
        crate::REMOTE_TARGET
    }

    pub fn is_local(&self) -> bool {
        matches!(crate::REMOTE_TARGET, RemoteTarget::Local)
    }

    pub fn firebase_assume_valid(&self) -> bool {
        self.is_local() && self.firebase_no_auth
    }

    pub fn jwt_decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_decoding_key.as_bytes())
    }
}

fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
