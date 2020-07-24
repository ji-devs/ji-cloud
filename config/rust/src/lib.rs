pub const MEDIA_UI_PATH:&'static str = "ui";
pub const MAX_SIGNIN_COOKIE:&'static str = "1209600"; // 2 weeks
pub const JSON_BODY_LIMIT:u64 = 16384; //1024 * 16
pub const COOKIE_DOMAIN:&'static str = "jicloud.org";
pub const CORS_ORIGINS:[&'static str;2] = ["https://jicloud.org", "https://sandbox.jicloud.org"];
pub const DB_POOL_CONNECTIONS:u32 = 5;

pub const REMOTE_DB_USER:&'static str = "postgres";
pub const REMOTE_DB_NAME:&'static str = "jicloud";
pub const SQL_PROXY_PORT:u16 = 6432; //must match the port number in build-utils/package.json where cloud-sql-proxy is launched

pub const DB_INSTANCE_SANDBOX:&'static str = "ji-cloud-developer-sandbox:europe-west1:ji-cloud-003-sandbox";
pub const DB_INSTANCE_RELEASE:&'static str = "ji-cloud:europe-west1:ji-cloud-002";

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}

impl RemoteTarget {
	pub fn google_credentials_env_name(&self) -> &'static str {
        match self {
            Self::Local => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
            Self::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
        }
    }
	
    pub fn api_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8081", 
            Self::Sandbox => "https://api.sandbox.jicloud.org",
            Self::Release => "https://api.jicloud.org",
        }
    }

    pub fn api_js_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8082", 
            Self::Sandbox => "https://api-js.sandbox.jicloud.org",
            Self::Release => "https://api-js.jicloud.org",
        }
    }

    pub fn media_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:4102", 
            Self::Sandbox | Self::Release => "https://media.jicloud.org",
        }
    }

	pub fn pages_url(&self) -> &'static str {
        match self {
            Self::Local => "http://localhost:8080", 
            Self::Sandbox => "https://sandbox.jicloud.org",
            Self::Release => "https://jicloud.org",
        }
    }
	
	pub fn frontend_url(&self) -> &'static str {
        match self {
            Self::Local | Self::Sandbox => "https://frontend.sandbox.jicloud.org",
            Self::Release => "https://frontend.jicloud.org",
        }
    }

    pub fn spa_url(&self, app:&str, path:&str) -> String {
        format!("{}/{}/{}", self.frontend_url(), app, path)
    }

    pub fn host(&self) -> Option<&'static str> {
        None
    }
}
