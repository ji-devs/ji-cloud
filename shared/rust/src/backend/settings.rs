
pub const MAX_SIGNIN_COOKIE:&'static str = "1209600"; // 2 weeks
pub const JSON_BODY_LIMIT:u64 = 16384; //1024 * 16
pub const COOKIE_DOMAIN:&'static str = "jicloud.org";
pub const CORS_ORIGINS:[&'static str;1] = ["https://jicloud.org"];

const REMOTE_DB_USER:&'static str = "postgres";
const REMOTE_DB_NAME:&'static str = "jicloud";
const SQL_PROXY_PORT:u32 = 6432; //must match the port number in build-utils/package.json where cloud-sql-proxy is launched

const DB_INSTANCE_SANDBOX:&'static str = "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox";
const DB_INSTANCE_RELEASE:&'static str = "ji-cloud:europe-west1:ji-cloud-002";

impl RemoteTarget {
    pub fn google_credentials_env_name(&self) -> &'static str {
        match self {
            Self::Local => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
        }
    }

    pub fn js_api(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8082",
            Self::Sandbox => "https://api-js.sandbox.jicloud.org",
            Self::Release => "https://api-js.jicloud.org",
        }
    }

    pub fn media_url_base(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4102",
            Self::Sandbox | Self::Release => "https://media.jicloud.org",
        }
    }

    pub fn frontend_url_base(&self) -> &'static str {
        match self {
            Self::Local | Self::Sandbox => "https://frontend.sandbox.jicloud.org",
            Self::Release => "https://frontend.jicloud.org",
        }
    }
}

// No need to set anything below here, the rest are helper functions to make things easier

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DbTarget {
    Local,
    Proxy,
    Remote(RemoteTarget),
}

pub fn db_connection_string(secret_db_pass:&str, db_target:DbTarget) -> String {
    match db_target {
        DbTarget::Local => {
            //these are env vars since it depends on developer's local machine
            let db_user = std::env::var("LOCAL_DB_USER").expect("When not using Cloud Sql Proxy, set LOCAL_DB_USER in .env");
            let db_pass = std::env::var("LOCAL_DB_PASS").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PASS in .env");
            let db_port = std::env::var("LOCAL_DB_PORT").expect("When not using Cloud Sql Proxy, set LOCAL_DB_PORT in .env");
            let db_name = std::env::var("LOCAL_DB_NAME").expect("When not using Cloud Sql Proxy, set LOCAL_DB_NAME in .env");
            format!("postgres://{}:{}@localhost:{}/{}", db_user, db_pass, db_port, db_name)
        },
        DbTarget::Proxy => {
            //The difference of sandbox vs. remote here depends on invoking cloud-sql-proxy
            format!("postgres://{}:{}@localhost:{}/{}", REMOTE_DB_USER, secret_db_pass, SQL_PROXY_PORT, REMOTE_DB_NAME)
        },
        DbTarget::Remote(remote_target) => {
            let instance_connection = 
                std::env::var("INSTANCE_CONNECTION_NAME")
                    .unwrap_or(match remote_target {
                        RemoteTarget::Sandbox => DB_INSTANCE_SANDBOX.to_string(),
                        RemoteTarget::Release => DB_INSTANCE_RELEASE.to_string(), 
                        _ => panic!("non-dev mode only makes sense for sandbox or release")
                    });

            let socket_path = std::env::var("DB_SOCKET_PATH").unwrap_or("/cloudsql".to_string());

            let full_socket_path = utf8_percent_encode(&format!("{}/{}", socket_path, instance_connection), NON_ALPHANUMERIC).to_string();

            let connection_string = format!("postgres://{}:{}@{}/{}", REMOTE_DB_USER, secret_db_pass, full_socket_path, REMOTE_DB_NAME);

            connection_string
        }
    }
}
