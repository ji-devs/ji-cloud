use std::{
    fmt,
};
use ji_cloud_shared::backend::{
    google::{get_secret, get_access_token_and_project_id},
    settings::{RemoteTarget, DbTarget, db_connection_string},
};
use cfg_if::cfg_if;

pub struct Settings {
    pub db_target: DbTarget,
    pub db_connection_string:String,
}

cfg_if! {
    if #[cfg(all(feature = "local", feature = "sqlproxy"))] {
        pub async fn init() -> Settings { 
            _init(RemoteTarget::Local, DbTarget::Proxy).await
        }
    } else if #[cfg(feature = "local")] {
        pub async fn init() -> Settings { 
            _init(RemoteTarget::Local, DbTarget::Local).await
        }
    } else if #[cfg(feature = "sandbox")] {
		pub async fn init() -> Settings { 
            _init(RemoteTarget::Sandbox, DbTarget::Remote(RemoteTarget::Sandbox)).await
        }
        
    } else if #[cfg(feature = "release")] {
        pub async fn init() -> Settings { 
            _init(RemoteTarget::Release, DbTarget::Remote(RemoteTarget::Release)).await
        }
    } else {
        pub async fn init() -> Settings { 
            panic!("no settings!");
        }
    } 
}


async fn _init(remote_target:RemoteTarget, db_target:DbTarget) -> Settings {
    let (token, project_id) = get_access_token_and_project_id(remote_target.google_credentials_env_name()).await.expect("couldn't get access token and project id!");

    let db_pass = get_secret(token.as_ref(), &project_id, "DB_PASS").await;

    match remote_target {
        RemoteTarget::Local => Settings::new_local(db_target, db_pass),
        RemoteTarget::Sandbox => Settings::new_sandbox(db_target, db_pass),
        RemoteTarget::Release => Settings::new_release(db_target, db_pass),
    }
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "db_target is [{:?}]", self.db_target)
    }
}

impl Settings {
    pub fn new_local(db_target:DbTarget, db_pass:String) -> Self {
        Self {
            db_target, 
            db_connection_string: db_connection_string(&db_pass, db_target),
        }
    }
    pub fn new_sandbox(db_target:DbTarget, db_pass:String) -> Self {
        Self {
            db_target, 
            db_connection_string: db_connection_string(&db_pass, db_target),
        }
    }
    pub fn new_release(db_target:DbTarget, db_pass:String) -> Self {
        Self {
            db_target, 
            db_connection_string: db_connection_string(&db_pass, db_target),
        }
    }
}
