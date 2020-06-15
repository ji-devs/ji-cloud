pub static CORS_ORIGINS:[&'static str;1] = ["https://jicloud.org"];
pub const MAX_SIGNIN_COOKIE:&'static str = "1209600"; // 2 weeks
pub const JSON_BODY_LIMIT:u64 = 16384; //1024 * 16
pub const HANDLEBARS_PATH:&'static str = "./handlebars";

use ji_cloud_shared::backend::settings;
use cfg_if::cfg_if;

pub use ji_cloud_shared::backend::settings::{SETTINGS, Settings, RemoteTarget};

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn init() { 
            settings::init(settings::RemoteTarget::Local);
        }
    } else if #[cfg(feature = "sandbox")] {
        pub fn init() { 
            settings::init(settings::RemoteTarget::Sandbox);
        }
    } else if #[cfg(feature = "release")] {
        pub fn init() { 
            settings::init(settings::RemoteTarget::Release);
        }
    } else {
        pub fn init() { 
        }
    } 
}
