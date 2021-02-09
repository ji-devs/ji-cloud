use once_cell::sync::OnceCell;
use std::fmt;
use cfg_if::cfg_if;
use config::RemoteTarget;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use shared::domain::auth::JWT_COOKIE_NAME;

pub static SETTINGS:OnceCell<Settings> = OnceCell::new();

#[derive(Clone)]
pub struct Settings {
    pub remote_target: RemoteTarget,
    pub firebase_dev: bool,
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
    #[wasm_bindgen(js_name = FRONTEND_DEV_AUTH)]
    fn frontend_dev_auth() -> bool;
    #[wasm_bindgen(js_name = API_TOKEN)]
    fn frontend_dev_token() -> String;
    #[wasm_bindgen(js_name = API_CSRF)]
    fn frontend_dev_csrf() -> String;
}

fn _init(remote_target:RemoteTarget) -> Settings {
    let settings = match remote_target {
        RemoteTarget::Local => Settings::new_local(),
        RemoteTarget::Sandbox => Settings::new_sandbox(),
        RemoteTarget::Release => Settings::new_release(),
    };

    if remote_target == RemoteTarget::Local {
        unsafe {
            if frontend_dev_auth() {
                let csrf = frontend_dev_csrf();
                let token = frontend_dev_token();
                super::storage::save_csrf_token(&csrf);
                web_sys::window()
                    .unwrap_throw()
                    .document()
                    .unwrap_throw()
                    .unchecked_into::<web_sys::HtmlDocument>()
                    .set_cookie(&format!("{}={}; PATH=/", JWT_COOKIE_NAME, token));
            }
        }
    }
    SETTINGS.set(settings.clone()).expect("couldn't set settings!");

    settings
}


impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "remote_target is [{:?}] and firebase_dev is [{:?}]", self.remote_target, self.firebase_dev)
    }
}

impl Settings {
    pub fn new_local() -> Self {
        Self {
            remote_target: RemoteTarget::Local,
            firebase_dev: true,
        }
    }
    pub fn new_sandbox() -> Self {
        Self {
            remote_target: RemoteTarget::Sandbox,
            firebase_dev: true,
        }
    }
    pub fn new_release() -> Self {
        Self {
            remote_target: RemoteTarget::Release,
            firebase_dev: false,
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
