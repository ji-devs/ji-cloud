use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::api::endpoints::{category, ApiEndpoint};

async fn todo() -> HttpResponse {
    todo!()
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(<category::Get as ApiEndpoint>::PATH, web::get().to(todo))
        .route(
            <category::Create as ApiEndpoint>::PATH,
            web::post().to(todo),
        )
        .route(
            <category::Update as ApiEndpoint>::PATH,
            web::patch().to(todo),
        )
        .route(
            <category::Delete as ApiEndpoint>::PATH,
            web::delete().to(todo),
        );
}
