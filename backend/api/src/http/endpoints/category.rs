use crate::{db, extractor::WrapAuthClaimsNoDb};
use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use shared::api::endpoints::{category, ApiEndpoint};
use shared::category::{
    CategoryCreateError, CategoryGetError, CategoryId, CategoryResponse, CreateCategoryRequest,
    NewCategoryResponse,
};
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

async fn create_category(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Json<<category::Create as ApiEndpoint>::Req>,
) -> actix_web::Result<
    Json<<category::Create as ApiEndpoint>::Res>,
    <category::Create as ApiEndpoint>::Err,
> {
    let CreateCategoryRequest { name, parent_id } = req.into_inner();

    let (id, index) = db::category::create(&db, &name, parent_id)
        .await
        .map_err(|_| CategoryCreateError::InternalServerError)?;

    Ok(Json(NewCategoryResponse { id, index }))
}

async fn delete_category(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<CategoryId>,
) -> actix_web::Result<HttpResponse, <category::Delete as ApiEndpoint>::Err> {
    db::category::delete(&db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().into())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        category::Get::PATH,
        category::Get::METHOD.route().to(get_categories),
    )
    .route(
        category::Create::PATH,
        category::Create::METHOD.route().to(create_category),
    )
    .route(
        category::Update::PATH,
        category::Update::METHOD.route().to(todo),
    )
    .route(
        category::Delete::PATH,
        category::Delete::METHOD.route().to(delete_category),
    );
}
