use crate::templates::{
    direct::direct_template_no_auth, epoch::epoch_page, info::info_template, passthrough, spa,
};
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn configure(config: &mut ServiceConfig) {
    // [Ty] Temporary redirect so that requests to /plans go to the JI wordpress site.
    async fn redirect() -> HttpResponse {
        let mut response = HttpResponse::TemporaryRedirect();
        response.append_header(("Location", "https://www.jewishinteractive.org/plans/"));
        response.into()
    }
    config
        .route("/kids/{path:.*}", web::get().to(spa::kids_template))
        .route("/kids", web::get().to(spa::kids_template))
        .route(
            "/classroom/{path:.*}",
            web::get().to(spa::classroom_template),
        )
        .route("/classroom", web::get().to(spa::classroom_template))
        .route("/community", web::get().to(spa::community_template))
        .route(
            "/community/{path:.*}",
            web::get().to(spa::community_template),
        )
        .route("/user", web::get().to(spa::user_template))
        .route("/user/{path:.*}", web::get().to(spa::user_template))
        .route("/admin/{path:.*}", web::get().to(spa::admin_template))
        .route("/admin", web::get().to(spa::admin_template))
        .route(
            "/asset/{page_kind}/{path:.*}",
            web::get().to(spa::asset_template),
        )
        // jig route is just to redirect old urls
        .route(
            "/jig/{page_kind}/{path:.*}",
            web::get().to(spa::asset_template),
        )
        .route("/legacy/play/{jig_id}", web::get().to(spa::legacy_template))
        .route(
            "/legacy/play/{jig_id}/{module_id}",
            web::get().to(spa::legacy_template_with_module),
        )
        .route(
            "/module/{kind}/{page_kind}/{path:.*}",
            web::get().to(spa::module_template),
        )
        .route("/dev/{path:.*}", web::get().to(spa::dev_template))
        .route("/", web::get().to(spa::home_template))
        .route("/home", web::get().to(spa::home_template))
        .route("/home/{path:.*}", web::get().to(spa::home_template))
        .route("/no-auth", web::get().to(direct_template_no_auth))
        .route("/info", web::get().to(info_template))
        .route("/epoch", web::get().to(epoch_page))
        .route("/plans", web::get().to(redirect))
        .route(
            "/service-worker.js",
            web::get().to(passthrough::service_worker),
        )
        .route("/manifest.json", web::get().to(passthrough::manifest))
        .route("/icon.png", web::get().to(passthrough::icon))
        .route("/icon-192x192.png", web::get().to(passthrough::icon_192))
        .route("/icon-512x512.png", web::get().to(passthrough::icon_512));
}
