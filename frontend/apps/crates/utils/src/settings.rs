use once_cell::sync::OnceCell;
use std::fmt;
use cfg_if::cfg_if;
use config::RemoteTarget;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shared::domain::auth::AUTH_COOKIE_NAME;

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
    let settings = match remote_target {
        RemoteTarget::Local => Settings::new_local(),
        RemoteTarget::Sandbox => Settings::new_sandbox(),
        RemoteTarget::Release => Settings::new_release(),
    };

    if remote_target == RemoteTarget::Local {
        unsafe {
            if dev_auth() {
                let csrf = dev_csrf();
                let token = dev_token();
                log::info!("manually setting auth for dev mode");

                super::storage::save_csrf_token(&csrf);


                web_sys::window()
                    .unwrap_throw()
                    .document()
                    .unwrap_throw()
                    .unchecked_into::<web_sys::HtmlDocument>()
                    .set_cookie(&format!("{}={}; PATH=/", AUTH_COOKIE_NAME, token));
            }
        }
    }
    SETTINGS.set(settings.clone()).expect("couldn't set settings!");

    settings
}


impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}]", self.remote_target)
    }
}

impl Settings {
    pub fn new_local() -> Self {
        Self {
            remote_target: RemoteTarget::Local,
        }
    }
    pub fn new_sandbox() -> Self {
        Self {
            remote_target: RemoteTarget::Sandbox,
        }
    }
    pub fn new_release() -> Self {
        Self {
            remote_target: RemoteTarget::Release,
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
