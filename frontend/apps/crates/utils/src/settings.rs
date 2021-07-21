use once_cell::sync::OnceCell;
use std::fmt;
use cfg_if::cfg_if;
use config::RemoteTarget;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shared::domain::auth::AUTH_COOKIE_NAME;
use crate::unwrap::UnwrapJiExt;

pub static SETTINGS:OnceCell<Settings> = OnceCell::new();

#[derive(Clone)]
pub struct Settings {
    pub remote_target: RemoteTarget,
}

cfg_if! {
    if #[cfg(feature = "local")] {
        pub fn init() -> Settings {
            _init(RemoteTarget::Local)
        }
    } else if #[cfg(feature = "sandbox")] {
		pub fn init() -> Settings { 
            _init(RemoteTarget::Sandbox)
        }
        
    } else if #[cfg(feature = "release")] {
        pub fn init() -> Settings { 
            _init(RemoteTarget::Release)
        }
    } else {
        pub fn init() -> Settings { 
            panic!("set a feature target!");
        }
    } 
}

fn _init(remote_target:RemoteTarget) -> Settings {
    let settings = Settings {
        remote_target
    };
   
    /*
    if remote_target == RemoteTarget::Local {
        unsafe {
            let window = web_sys::window().unwrap_ji();
            if dev_auth() && !window.location().pathname().unwrap_ji().contains("user/"){

                let csrf = dev_csrf();
                let token = dev_token();
                log::info!("manually setting auth for dev mode");

                super::storage::save_csrf_token(&csrf);

                window
                    .document()
                    .unwrap_ji()
                    .unchecked_into::<web_sys::HtmlDocument>()
                    .set_cookie(&format!("{}={}; Path=/", AUTH_COOKIE_NAME, token));
            } else {
                log::info!("skipping auth for dev mode");
            }
        }
    }
    */
    SETTINGS.set(settings.clone()).expect_ji("couldn't set settings!");

    settings
}


impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}]", self.remote_target)
    }
}
