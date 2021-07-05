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

//These will only be set in the local index.html created via dev-files
//However they are only called in local mode, so it's fine
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = DEV_AUTH)]
    fn dev_auth() -> bool;
    #[wasm_bindgen(js_name = API_TOKEN)]
    fn dev_token() -> String;
    #[wasm_bindgen(js_name = API_CSRF)]
    fn dev_csrf() -> String;
}

fn _init(remote_target:RemoteTarget) -> Settings {
    let settings = Settings {
        remote_target
    };
    
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
                    .set_cookie(&format!("{}={}; PATH=/", AUTH_COOKIE_NAME, token));
            } else {
                log::info!("skipping auth for dev mode");
            }
        }
    }
    SETTINGS.set(settings.clone()).expect_ji("couldn't set settings!");

    settings
}


impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}]", self.remote_target)
    }
}