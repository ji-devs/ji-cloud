use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use ji_cloud_shared::backend::google::{get_secret, get_access_token_and_project_id};
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::loader::{load_string, load_json};
use crate::settings::{SETTINGS, RemoteTarget};

#[derive(Serialize, Deserialize)]
struct Info {
    Secret: String,
    Roles: Vec<Role>
}

#[derive(Serialize, Deserialize)]
struct Role {
    Id: u32,
    name: String,
    about: String
}

pub async fn info_template(hb:Arc<Handlebars<'_>>) -> Result<impl warp::Reply, warp::Rejection> {

    let (token, project_id) = get_access_token_and_project_id(match SETTINGS.get().unwrap().db_target {
        RemoteTarget::Local => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
        RemoteTarget::Sandbox => "GOOGLE_APPLICATION_CREDENTIALS_DEV_SANDBOX",
        RemoteTarget::Release => "GOOGLE_APPLICATION_CREDENTIALS_DEV_RELEASE",
    }).await.expect("couldn't get access token and project id!");


    let secret_test = get_secret(token.as_ref(), &project_id, "SANITY_TEST").await;
    let info = Info {
        Secret: secret_test,
        Roles: vec![]
    };

    let render = hb.render("info", &info).unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}
