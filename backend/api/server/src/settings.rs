use jsonwebtoken::EncodingKey;
use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ji_cloud_shared::backend::{
    google::{get_secret, get_access_token_and_project_id},
    settings::{RemoteTarget, DbTarget, DbCredentials},
};
use once_cell::sync::OnceCell;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use cfg_if::cfg_if;


pub static SETTINGS:OnceCell<Settings> = OnceCell::new();

pub struct Settings {
    pub remote_target: RemoteTarget,
    pub db_target: DbTarget,
    pub local_insecure: bool,
    pub port: u16,
    pub epoch: Duration,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    pub jwt_decoding_key:String,
    pub inter_server_secret:String,
    pub db_credentials:DbCredentials,
}

cfg_if! {
    if #[cfg(all(feature = "local", feature = "sqlproxy"))] {
        pub async fn init() { 
            _init(RemoteTarget::Local, DbTarget::Proxy).await;
        }
    } else if #[cfg(feature = "local")] {
        pub async fn init() { 
            _init(RemoteTarget::Local, DbTarget::Local).await;
        }
    } else if #[cfg(feature = "sandbox")] {
		pub async fn init() { 
            _init(RemoteTarget::Sandbox, DbTarget::Remote(RemoteTarget::Sandbox)).await;
        }
        
    } else if #[cfg(feature = "release")] {
        pub async fn init() { 
            _init(RemoteTarget::Release, DbTarget::Remote(RemoteTarget::Release)).await;
        }
    } else {
        pub async fn init() { 
        }
    } 
}


async fn _init(remote_target:RemoteTarget, db_target:DbTarget) {
    actions::settings::init().await;

    log::info!("initializing settings for remote_target: {:?}, db_target: {:?}", remote_target, db_target);

    let (token, project_id) = get_access_token_and_project_id(remote_target.google_credentials_env_name()).await.expect("couldn't get access token and project id!");

    let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
    let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;
    let inter_server_secret = get_secret(token.as_ref(), &project_id, "INTER_SERVER").await;


    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal

    SETTINGS.set(match remote_target {
        RemoteTarget::Local => Settings::new_local(db_target, jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
        RemoteTarget::Sandbox => Settings::new_sandbox(db_target, jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
        RemoteTarget::Release => Settings::new_release(db_target, jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
    }).expect("couldn't set settings!");
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}]. port is [{}]", self.remote_target, self.port)
    }
}
fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}



impl Settings {
    pub fn new_local(db_target:DbTarget, jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, db_pass:String) -> Self {
        Self {
            remote_target: RemoteTarget::Local,
            db_target, 
            local_insecure: true,
            port: 8081,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_credentials: DbCredentials::new(&db_pass, db_target),
        }
    }
    pub fn new_sandbox(db_target:DbTarget, jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, db_pass:String) -> Self {
        Self {
            remote_target: RemoteTarget::Sandbox,
            db_target, 
            port: 8080,
            local_insecure: false,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_credentials: DbCredentials::new(&db_pass, db_target),
        }
    }
    pub fn new_release(db_target:DbTarget, jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, db_pass:String) -> Self {
        Self {
            remote_target: RemoteTarget::Release,
            db_target, 
            port: 8080,
            local_insecure: false,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_credentials: DbCredentials::new(&db_pass, db_target),
        }
    }
}
