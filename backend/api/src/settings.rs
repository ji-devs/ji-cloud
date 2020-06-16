pub const MAX_SIGNIN_COOKIE:&'static str = "1209600"; // 2 weeks
pub const JSON_BODY_LIMIT:u64 = 16384; //1024 * 16
pub const COOKIE_DOMAIN:&'static str = "jicloud.org";
pub static CORS_ORIGINS:[&'static str;1] = ["https://jicloud.org"];

use jsonwebtoken::EncodingKey;
use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ji_cloud_shared::backend::google::{get_secret, get_access_token_and_project_id};
use once_cell::sync::OnceCell;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use cfg_if::cfg_if;


#[derive(Debug, PartialEq, Eq)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}

pub static SETTINGS:OnceCell<Settings> = OnceCell::new();

pub struct Settings {
    pub auth_target: RemoteTarget,
    pub db_target: RemoteTarget,
    pub media_url_base: &'static str,
    pub local_insecure: bool,
    pub port: u16,
    pub epoch: Duration,
    pub jwt_encoding_key: EncodingKey,
    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    pub jwt_decoding_key:String,
    pub inter_server_secret:String,
    pub db_connection_string:String,
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


async fn _init(target:RemoteTarget) {
    log::info!("initializing settings for {:?}", target);

    let (token, project_id) = get_access_token_and_project_id(match target {
        RemoteTarget::Local => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
        RemoteTarget::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
        RemoteTarget::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
    }).await.expect("couldn't get access token and project id!");

    let jwt_secret = get_secret(token.as_ref(), &project_id, "JWT_SECRET").await;
    let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;
    let inter_server_secret = get_secret(token.as_ref(), &project_id, "INTER_SERVER").await;


    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal

    SETTINGS.set(match target {
        RemoteTarget::Local => Settings::new_local(jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
        RemoteTarget::Sandbox => Settings::new_sandbox(jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
        RemoteTarget::Release => Settings::new_release(jwt_encoding_key, jwt_secret, inter_server_secret, db_pass),
    }).expect("couldn't set settings!");
}

cfg_if! {
    if #[cfg(feature = "sqlproxy")] {
        fn local_db_connection_string(secret_db_pass:&str) -> String {
            //note - the port number must match the one in build-utils/package.json 
            //where cloud_sql_proxy is launched - and this must not conflict with 
            //any other local services
            format!("postgres://postgres:{}@localhost:6432/jicloud", secret_db_pass)
        }
    } else {
        fn local_db_connection_string(unused_secret_db_pass:&str) -> String {
            let db_user = std::env::var("LOCAL_DB_USER").expect("When not using Cloud Sql Proxy, set LOCAL_DB_USER in .env");
            let db_pass = std::env::var("LOCAL_DB_PASS").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PASS in .env");
            let db_port = std::env::var("LOCAL_DB_PORT").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PORT in .env");
            let db_name = std::env::var("LOCAL_DB_NAME").expect("When not using Cloud Sql Proxy, set LOCAL_DB_NAME in .env");
            format!("postgres://{}:{}@localhost:{}/{}", db_user, db_pass, db_port, db_name)
        }
    }
}

fn remote_db_connection_string(db_pass:&str, db_target:RemoteTarget) -> String {
    let instance_connection = std::env::var("INSTANCE_CONNECTION_NAME").unwrap_or(
        match db_target {
            RemoteTarget::Sandbox => "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox",
            RemoteTarget::Release => "ji-cloud:europe-west1:ji-cloud-002",
            _ => ""
        }.to_string()
    );
    let socket_path = std::env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

    let full_socket_path = utf8_percent_encode(&format!("{}/{}", socket_path, instance_connection), NON_ALPHANUMERIC).to_string();

    let db_user = "postgres";
    let db_name = "jicloud";
    let connection_string = format!("postgres://{}:{}@{}/{}", db_user, db_pass, full_socket_path, db_name);

    connection_string
}

impl Settings {
    pub fn js_api(&self) -> &'static str {
        match self.auth_target {
            RemoteTarget::Local => "http://localhost:8082",
            RemoteTarget::Sandbox=> "https://sandbox.api-js.jicloud.org",
            RemoteTarget::Release=> "https://api-js.jicloud.org",
        }
    }
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "auth_target is [{:?}] and db_target is [{:?}]. port is [{}]", self.auth_target, self.db_target, self.port)
    }
}
fn get_epoch() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}



    //SETTINGS.set(Settings::new(jwt_encoding_key, jwt_secret, inter_server_secret, db_pass));
impl Settings {
    pub fn new_local(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, remote_db_pass:String) -> Self {
        Self {
            auth_target: RemoteTarget::Local,
            db_target: RemoteTarget::Local,
            media_url_base: "http://localhost:4102",
            local_insecure: true,
            port: 8081,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_connection_string: local_db_connection_string(&remote_db_pass),
        }
    }
    pub fn new_sandbox(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, db_pass:String) -> Self {
        Self {
            auth_target: RemoteTarget::Sandbox,
            db_target: RemoteTarget::Sandbox,
            media_url_base: "https://storage.googleapis.com/ji-cloud-eu",
            port: 8080,
            local_insecure: false,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_connection_string: remote_db_connection_string(&db_pass, RemoteTarget::Sandbox),
        }
    }
    pub fn new_release(jwt_encoding_key:EncodingKey, jwt_decoding_key: String, inter_server_secret:String, db_pass:String) -> Self {
        Self {
            auth_target: RemoteTarget::Release,
            db_target: RemoteTarget::Release,
            media_url_base: "https://storage.googleapis.com/ji-cloud-eu",
            port: 8080,
            local_insecure: false,
            epoch: get_epoch(),
            jwt_encoding_key,
            jwt_decoding_key,
            inter_server_secret,
            db_connection_string: remote_db_connection_string(&db_pass, RemoteTarget::Release),
        }
    }

    pub fn spa_url(&self, app:&str, path:&str) -> String {
        format!("{}/spa/{}/{}", self.media_url_base, app, path)
    }
    
}
