use std::sync::Arc;
use handlebars::Handlebars;
use serde_json::json;
use futures_util::future::TryFutureExt;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};
use crate::settings::SETTINGS;
use crate::reject::{CustomWarpRejection, RequiredData};
use crate::loader::{load_string, load_json};

#[derive(Eq, PartialEq, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum DirectPage {
    Home,
    NoAuth,
    NotFound,
}

pub async fn direct_template(hb:Arc<Handlebars<'_>>, direct:DirectPage) -> Result<impl warp::Reply, warp::Rejection> {


    let render = match direct {
        DirectPage::Home => hb.render("home", &()),
        DirectPage::NoAuth=> hb.render("no-auth", &()),
        _ => hb.render("not-found", &()),
    }.unwrap_or_else(|err| err.to_string());

    Ok(warp::reply::html(render))
}
