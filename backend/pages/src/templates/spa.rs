use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use crate::settings::SETTINGS;
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::loader::{load_string, load_json};

#[derive(Eq, PartialEq, strum_macros::Display, strum_macros::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum SpaPage {
    User
}

#[derive(Serialize, Deserialize)]
struct PageInfo {
    AppJs: String,
    Firebase: bool,
}

pub async fn spa_template(hb:Arc<Handlebars<'_>>, spa:SpaPage) -> Result<impl warp::Reply, warp::Rejection> {

    let info = PageInfo {
        AppJs: SETTINGS.get().unwrap().spa_url(spa.as_ref(), "js/index.js"),
        Firebase: match spa {
            SpaPage::User => true,
            _ => false
        }
    };

    let render = hb.render("spa", &info).unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}
