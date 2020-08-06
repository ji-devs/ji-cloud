use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use config::RemoteTarget;
use core::settings::Settings;

use askama::Template;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SpaPage {
    User,
}

impl SpaPage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
        }
    }
}

#[derive(Template)]
#[template(path = "spa.html")]
struct SpaPageInfo {
    app_js: String,
    firebase: bool,
    local_dev: bool,
}

fn spa_template(settings: &Settings, spa: SpaPage) -> actix_web::Result<HttpResponse> {
    let info = SpaPageInfo {
        app_js: settings.remote_target.spa_url(spa.as_str(), "js/index.js"),
        firebase: matches!(spa, SpaPage::User),
        local_dev: matches!(settings.remote_target, RemoteTarget::Local),
    };

    let info = info.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().body(info))
}

pub async fn spa_user_template(settings: Data<Settings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::User)
}
