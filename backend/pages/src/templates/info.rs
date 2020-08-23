use actix_web::{error::ErrorInternalServerError, web::Data, HttpResponse};
use askama::Template;
use core::google::{get_access_token_and_project_id, get_secret};
use core::settings::RuntimeSettings;

struct Role {
    _id: u32,
    _name: String,
    _about: String,
}

#[derive(Template)]
#[template(path = "info.html")]
struct Info {
    secret: String,
    _roles: Vec<Role>,
    local_dev: bool,
}

pub async fn info_template(settings: Data<RuntimeSettings>) -> actix_web::Result<HttpResponse> {
    let (token, project_id) =
        get_access_token_and_project_id(settings.remote_target().google_credentials_env_name())
            .await
            .expect("couldn't get access token and project id!");

    let secret_test = get_secret(token.as_ref(), &project_id, "SANITY_TEST")
        .await
        .map_err(ErrorInternalServerError)?;

        
    let info = Info {
        secret: secret_test,
        _roles: vec![],
        local_dev: settings.is_local(),
    };

    let info = info.render().map_err(ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().body(info))
}
