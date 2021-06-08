use actix_web::http::StatusCode;
use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use askama::Template;
use core::settings::RuntimeSettings;

#[derive(Template)]
#[template(path = "title.html")]
struct TitlePage<'a> {
    local_dev: bool,
    title: &'a str,
}

#[deprecated]
#[allow(dead_code)]
pub async fn direct_template_home(
    settings: Data<RuntimeSettings>,
) -> actix_web::Result<HttpResponse> {
    direct_template(&settings, "Home!", StatusCode::OK)
}

fn direct_template(
    settings: &RuntimeSettings,
    text: &str,
    status: StatusCode,
) -> actix_web::Result<HttpResponse> {
    let page = TitlePage {
        local_dev: settings.is_local(),
        title: text,
    };

    let page = page.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::build(status).body(page))
}

pub async fn direct_template_404(
    settings: Data<RuntimeSettings>,
) -> actix_web::Result<HttpResponse> {
    direct_template(&settings, "Not Found!", StatusCode::NOT_FOUND)
}

pub async fn direct_template_no_auth(
    settings: Data<RuntimeSettings>,
) -> actix_web::Result<HttpResponse> {
    direct_template(&settings, "No Auth!", StatusCode::UNAUTHORIZED)
}
