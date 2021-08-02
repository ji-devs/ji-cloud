use actix_web::http::{header, Method};
use shared::config::CORS_ORIGINS;

pub fn get(local_insecure: bool) -> actix_cors::Cors {
    let mut cors = actix_cors::Cors::default()
        .supports_credentials()
        .allowed_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .expose_headers(&[
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-csrf"),
        ]);

    if local_insecure {
        cors = cors.allow_any_origin();
    } else {
        for origin in CORS_ORIGINS {
            cors = cors.allowed_origin(origin);
        }
    }

    cors
}
