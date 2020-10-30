use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use core::settings::RuntimeSettings;

use askama::Template;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SpaPage {
    User,
    Admin,
    Jig,
}

impl SpaPage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
            Self::Jig => "jig",
        }
    }
}

#[derive(Template)]
#[template(path = "spa.html")]
struct SpaPageInfo {
    app_js: String,
    app_css: String,
    firebase: bool,
    google_maps_url: Option<String>,
    local_dev: bool,
}

fn spa_template(settings: &RuntimeSettings, spa: SpaPage) -> actix_web::Result<HttpResponse> {
    let google_maps_url = match spa {
        // todo: `Cow::borrowed` ('static)
        SpaPage::User => Some(settings.remote_target().google_maps_url().to_owned()),
        _ => None,
    };

    let info = SpaPageInfo {
        app_js: settings
            .remote_target()
            .spa_url(spa.as_str(), "js/index.js"),
        app_css: settings.remote_target().css_url(true),
        firebase: matches!(spa, SpaPage::User),
        google_maps_url,
        local_dev: settings.is_local(),
    };

    let info = info.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().body(info))
}

pub async fn user_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::User)
}

pub async fn admin_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Admin)
}

pub async fn jig_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    spa_template(&settings, SpaPage::Jig)
}
