use config::CORS_ORIGINS;
use core::settings::SETTINGS;
use warp::http::Method;

pub fn get_cors() -> warp::filters::cors::Builder {
    let builder = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec!["Authorization", "Content-Type", "X-CSRF"])
        .allow_credentials(true);

    if SETTINGS.get().unwrap().local_insecure {
        builder.allow_any_origin()
    } else {
        builder.allow_origins(CORS_ORIGINS.iter().cloned())
    }
}
