use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use ji_cloud_shared::backend::google::{get_secret, get_google_token, get_google_credentials};
use crate::settings::SETTINGS;
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::loader::{load_string, load_json};
use crate::db::PgPool;

#[derive(Serialize, Deserialize)]
struct Info {
    TokenSanity: String
}

pub async fn info_template(hb:Arc<Handlebars<'_>>, pool:PgPool) -> Result<impl warp::Reply, warp::Rejection> {

    let credentials = get_google_credentials().await;
    let token = get_google_token(&credentials).await;

    let token_sanity = get_secret(&token, &credentials.project_id, "SANITY_TEST").await;
    let info = Info {
        TokenSanity: token_sanity,
    };

    let render = hb.render("info", &info).unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}
