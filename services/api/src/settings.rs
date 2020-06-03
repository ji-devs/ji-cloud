use std::fmt;
use cfg_if::cfg_if;
use lazy_static::lazy_static;
use strum_macros::{Display, EnumString};
use jsonwebtoken::{EncodingKey, DecodingKey};

lazy_static! {
    pub static ref SETTINGS:Settings = Settings::new();
    pub static ref JWT_ENCODING_KEY:EncodingKey = {
        let secret = std::env::var("JWT_SECRET").expect("must have JWT_SECRET set");
        EncodingKey::from_secret(secret.as_ref())
    };

    //TODO see: https://github.com/Keats/jsonwebtoken/issues/120#issuecomment-634096881
    //Keeping a string is a stop-gap measure for now, not ideal
    pub static ref JWT_DECODING_KEY:String = {
        let secret = std::env::var("JWT_SECRET").expect("must have JWT_SECRET set");

        secret
        //DecodingKey::from_secret(secret.as_ref())
    };
    pub static ref SHARED_SERVER_SECRET:String = std::env::var("SHARED_SERVER_SECRET").expect("must have SHARED_SERVER_SECRET set");
    pub static ref DB_CONNECTION:String = std::env::var("DATABASE_URL").expect("must have DATABASE_URL set");

    pub static ref CORS_ORIGINS:Vec<&'static str> = vec!["https://jicloud.org"];
}

pub const MAX_SIGNIN_COOKIE:&'static str = "1209600"; // 2 weeks
pub const JSON_BODY_LIMIT:u64 = 16384; //1024 * 16
pub const COOKIE_DOMAIN:&'static str = "jicloud.org";

pub struct Settings {
    pub auth_target: RemoteTarget,
    pub db_target: RemoteTarget,
    pub local_insecure: bool,
    pub port: u16,
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
        write!(f, "auth_target is [{}] and db_target is [{}]. port is [{}]", self.auth_target, self.db_target, self.port)
    }
}

#[derive(Display, EnumString, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum RemoteTarget {
    Local,
    Sandbox,
    Release,
}


impl Settings {
    pub fn new_local() -> Self {
        Self {
            auth_target: RemoteTarget::Local,
            db_target: RemoteTarget::Local,
            local_insecure: true,
            port: 8081,
        }
    }
    pub fn new_sandbox() -> Self {
        Self {
            auth_target: RemoteTarget::Sandbox,
            db_target: RemoteTarget::Sandbox,
            port: 8080,
            local_insecure: false,
        }
    }
    pub fn new_release() -> Self {
        Self {
            auth_target: RemoteTarget::Release,
            db_target: RemoteTarget::Release,
            port: 8080,
            local_insecure: false,
        }
    }
    
    cfg_if! {
        if #[cfg(feature = "local")] {
            pub fn new() -> Self { Self::new_local() }
        } else if #[cfg(feature = "sandbox")] {
            pub fn new() -> Self { Self::new_sandbox() }
        } else if #[cfg(feature = "release")] {
            pub fn new() -> Self { Self::new_release() }
        } else {
            pub fn new() -> Self { unimplemented!() }
        } 
    }
}
