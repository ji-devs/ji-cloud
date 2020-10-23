use crate::templates::{
    direct::{direct_template_home, direct_template_no_auth},
    epoch::epoch_page,
    info::info_template,
    spa,
};
use actix_web::web::{self, ServiceConfig};

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/user.*", web::get().to(spa::user_template))
        .route("/user", web::get().to(spa::user_template))
        .route("/admin.*", web::get().to(spa::admin_template))
        .route("/admin", web::get().to(spa::admin_template))
        .route("/jig.*", web::get().to(spa::jig_template))
        .route("/jig", web::get().to(spa::jig_template))
        .route("/", web::get().to(direct_template_home))
        .route("/no-auth", web::get().to(direct_template_no_auth))
        .route("/info", web::get().to(info_template))
        .route("/epoch", web::get().to(epoch_page));
}
