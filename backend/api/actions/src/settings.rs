use jsonwebtoken::EncodingKey;
use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ji_cloud_shared::backend::{
    google::{get_secret, get_access_token_and_project_id},
    settings::RemoteTarget,
};
use once_cell::sync::OnceCell;
use cfg_if::cfg_if;


pub static SETTINGS:OnceCell<Settings> = OnceCell::new();

pub struct Settings {
    pub remote_target: RemoteTarget,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    pub jwt_decoding_key:String,
    pub inter_server_secret:String,
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub async fn init() { 
            _init(RemoteTarget::Local).await;
        }
    } else if #[cfg(feature = "sandbox")] {
		pub async fn init() { 
            _init(RemoteTarget::Sandbox).await;
        }
        
    } else if #[cfg(feature = "release")] {
        pub async fn init() { 
            _init(RemoteTarget::Release).await;
        }
    } else {
        pub async fn init() { 
        }
    } 
}


async fn _init(remote_target:RemoteTarget) {

    let (token, project_id) = get_access_token_and_project_id(remote_target.google_credentials_env_name()).await.expect("couldn't get access token and project id!");

    let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
    let inter_server_secret = get_secret(token.as_ref(), &project_id, "INTER_SERVER").await;


    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal

    SETTINGS.set(match remote_target {
        RemoteTarget::Local => Settings::new_local(jwt_encoding_key, jwt_secret, inter_server_secret),
        RemoteTarget::Sandbox => Settings::new_sandbox(jwt_encoding_key, jwt_secret, inter_server_secret),
        RemoteTarget::Release => Settings::new_release(jwt_encoding_key, jwt_secret, inter_server_secret),
    }).expect("couldn't set settings!");
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}]", self.remote_target)
    }
}


impl Settings {
    pub fn new_local(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String) -> Self {
        Self {
            remote_target: RemoteTarget::Local,
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
        }
    }
    pub fn new_sandbox(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String) -> Self {
        Self {
            remote_target: RemoteTarget::Sandbox,
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
        }
    }
    pub fn new_release(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String) -> Self {
        Self {
            remote_target: RemoteTarget::Release,
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
        }
    }
}
