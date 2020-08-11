use actix_web::web::ServiceConfig;

mod meta;

pub fn configure(cfg: &mut ServiceConfig) {
    meta::configure(cfg);
}
