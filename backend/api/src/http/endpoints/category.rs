use crate::{db, extractor::WrapAuthClaimsNoDb};
use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
};
use serde_qs::actix::{QsQuery, QsQueryConfig};
use shared::api::endpoints::{category, ApiEndpoint};
use shared::category::{
    CategoryId, CategoryResponse, CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest,
    NewCategoryResponse, UpdateCategoryRequest,
};
use sqlx::PgPool;

async fn get_categories(
    db: Data<PgPool>,
    // _claims: WrapAuthClaimsNoDb,
    req: Option<QsQuery<<category::Get as ApiEndpoint>::Req>>,
) -> actix_web::Result<Json<<category::Get as ApiEndpoint>::Res>, <category::Get as ApiEndpoint>::Err>
{
    let req = req.map_or_else(GetCategoryRequest::default, QsQuery::into_inner);

    db::category::get_subtree(&db, &req.ids).await?;

    let categories = match req.scope {
        Some(CategoryTreeScope::Decendants) if req.ids.is_empty() => {
            db::category::get_tree(&db).await?
        }
        Some(CategoryTreeScope::Ancestors) | None if req.ids.is_empty() => {
            db::category::get_top_level(&db).await?
        }
        Some(CategoryTreeScope::Decendants) => db::category::get_subtree(&db, &req.ids).await?,
        Some(CategoryTreeScope::Ancestors) => {
            db::category::get_ancestor_tree(&db, &req.ids).await?
        }
        None => db::category::get_exact(&db, &req.ids).await?,
    };

    Ok(Json(CategoryResponse { categories }))
}

async fn create_category(
    db: Data<PgPool>,
    // _claims: WrapAuthClaimsNoDb,
    req: Json<<category::Create as ApiEndpoint>::Req>,
) -> actix_web::Result<
    Json<<category::Create as ApiEndpoint>::Res>,
    <category::Create as ApiEndpoint>::Err,
> {
    let CreateCategoryRequest { name, parent_id } = req.into_inner();

    let (id, index) = db::category::create(&db, &name, parent_id).await?;

    Ok(Json(NewCategoryResponse { id, index }))
}

async fn update_category(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    req: Option<Json<<category::Update as ApiEndpoint>::Req>>,
    path: web::Path<CategoryId>,
) -> actix_web::Result<HttpResponse, <category::Update as ApiEndpoint>::Err> {
    let UpdateCategoryRequest {
        name,
        parent_id,
        index,
    } = req.map_or_else(Default::default, Json::into_inner);

    db::category::update(
        &db,
        path.into_inner(),
        parent_id,
        name.as_deref(),
        index.map(|it| it as i16),
    )
    .await?;

    Ok(HttpResponse::NoContent().into())
}

async fn delete_category(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
    path: web::Path<CategoryId>,
) -> actix_web::Result<HttpResponse, <category::Delete as ApiEndpoint>::Err> {
    db::category::delete(&db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().into())
}

fn qs_array_cfg() -> QsQueryConfig {
    QsQueryConfig::default().qs_config(serde_qs::Config::new(2, false))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource(category::Get::PATH)
            .app_data(qs_array_cfg())
            .route(category::Get::METHOD.route().to(get_categories))
            .route(category::Create::METHOD.route().to(create_category)),
    )
    .route(
        category::Update::PATH,
        category::Update::METHOD.route().to(update_category),
    )
    .route(
        category::Delete::PATH,
        category::Delete::METHOD.route().to(delete_category),
    );
}
