use actix_web::http::{header, Method};
use config::CORS_ORIGINS;

pub fn get(local_insecure: bool) -> actix_cors::Cors {
    let mut cors = actix_cors::Cors::new()
        .allowed_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allowed_headers(&[
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-csrf"),
        ]);

    if !local_insecure {
        for origin in CORS_ORIGINS.iter() {
            cors = cors.allowed_origin(origin);
        }
    }

    cors
}
