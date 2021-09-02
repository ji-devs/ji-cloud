use crate::templates::{
    direct::direct_template_no_auth, epoch::epoch_page, info::info_template, spa,
};
use actix_web::web::{self, ServiceConfig};

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/kids.*", web::get().to(spa::kids_template))
        .route("/kids", web::get().to(spa::kids_template))
        .route("/user.*", web::get().to(spa::user_template))
        .route("/user", web::get().to(spa::user_template))
        .route("/admin.*", web::get().to(spa::admin_template))
        .route("/admin", web::get().to(spa::admin_template))
        .route(
            "/jig/{page_kind}/{jig_id}",
            web::get().to(spa::jig_template),
        )
        .route(
            "/jig/{page_kind}/{jig_id}/{module_id}",
            web::get().to(spa::jig_template_with_module),
        )
        .route("/legacy/play/{jig_id}", web::get().to(spa::legacy_template))
        .route(
            "/legacy/play/{jig_id}/{module_id}",
            web::get().to(spa::legacy_template_with_module),
        )
        .route(
            "/module/{kind}/{page_kind}/{jig_id}/{module_id}",
            web::get().to(spa::module_template),
        )
        .route("/dev/{path:.*}", web::get().to(spa::dev_template))
        .route("/", web::get().to(spa::home_template))
        .route("/home", web::get().to(spa::home_template))
        .route("/home/{path:.*}", web::get().to(spa::home_template))
        .route("/no-auth", web::get().to(direct_template_no_auth))
        .route("/info", web::get().to(info_template))
        .route("/epoch", web::get().to(epoch_page));
}
