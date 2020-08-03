use actix_web::{web::ServiceConfig, HttpResponse};
use shared::api::endpoints::{category, ApiEndpoint};

async fn todo() -> HttpResponse {
    todo!()
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(category::Get::PATH, category::Get::METHOD.route().to(todo))
        .route(
            category::Create::PATH,
            category::Create::METHOD.route().to(todo),
        )
        .route(
            category::Update::PATH,
            category::Update::METHOD.route().to(todo),
        )
        .route(
            category::Delete::PATH,
            category::Delete::METHOD.route().to(todo),
        );
}
