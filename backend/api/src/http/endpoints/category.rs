use crate::{db, extractor::WrapAuthClaimsNoDb};
use actix_web::{
    web::{Data, Json, ServiceConfig},
    HttpResponse,
};
use shared::api::endpoints::{category, ApiEndpoint};
use shared::category::{CategoryGetError, CategoryResponse};
use sqlx::PgPool;

async fn todo() -> HttpResponse {
    todo!()
}

async fn get_categories(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
) -> actix_web::Result<Json<<category::Get as ApiEndpoint>::Res>, <category::Get as ApiEndpoint>::Err>
{
    db::category::get(&db)
        .await
        .map_err(|_| CategoryGetError::InternalServerError)
        .map(|it| Json(CategoryResponse { categories: it }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        category::Get::PATH,
        category::Get::METHOD.route().to(get_categories),
    )
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
