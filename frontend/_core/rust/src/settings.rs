use once_cell::sync::OnceCell;
use std::fmt;
use cfg_if::cfg_if;


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


fn _init(remote_target:RemoteTarget) -> Settings {
    let settings = match remote_target {
        RemoteTarget::Local => Settings::new_local(),
        RemoteTarget::Sandbox => Settings::new_sandbox(),
        RemoteTarget::Release => Settings::new_release(),
    };
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
