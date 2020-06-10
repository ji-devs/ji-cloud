use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use ji_cloud_shared::backend::google::{get_secret, get_access_token_and_project_id};
use crate::settings::SETTINGS;
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::loader::{load_string, load_json};
use crate::db::PgPool;

#[derive(Serialize, Deserialize)]
struct Info {
    TokenSanity: String
}

pub async fn info_template(hb:Arc<Handlebars<'_>>, pool:PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let (token, project_id) = get_access_token_and_project_id().await;

    let token_sanity = get_secret(token.as_ref(), &project_id, "SANITY_TEST").await;
    let info = Info {
        TokenSanity: token_sanity,
    };

    let render = hb.render("info", &info).unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}
