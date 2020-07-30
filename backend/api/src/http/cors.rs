use actix_web::http::{header, Method};
use config::CORS_ORIGINS;
use core::settings::SETTINGS;

pub fn get_cors_actix() -> actix_cors::Cors {
    let mut cors = actix_cors::Cors::new()
        .allowed_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allowed_headers(&[
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("X-CSRF"),
        ]);

    if !SETTINGS.get().unwrap().local_insecure {
        for origin in CORS_ORIGINS.iter() {
            cors = cors.allowed_origin(origin);
        }
    }

    cors
}
