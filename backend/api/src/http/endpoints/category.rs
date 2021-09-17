use actix_web::{
    web::{self, Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use shared::{
    api::endpoints::{category, ApiEndpoint},
    domain::category::{
        CategoryId, CategoryResponse, CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest,
        NewCategoryResponse, UpdateCategoryRequest,
    },
};
use sqlx::PgPool;

use crate::{
    db, error,
    extractor::{ScopeManageCategory, TokenUserWithScope},
};

pub enum CreateError {
    ParentCategoryNotFound,
    InternalServerError(anyhow::Error),
}

impl<T: Into<anyhow::Error>> From<T> for CreateError {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for CreateError {
    fn into(self) -> actix_web::Error {
        match self {
            Self::ParentCategoryNotFound => error::BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Parent Category Not Found".to_owned(),
            )
            .into(),
            Self::InternalServerError(e) => crate::error::ise(e),
        }
    }
}

/// Get a tree of categories.
async fn get_categories(
    db: Data<PgPool>,
    req: Option<Query<<category::Get as ApiEndpoint>::Req>>,
) -> actix_web::Result<Json<<category::Get as ApiEndpoint>::Res>, error::Server> {
    let req = req.map_or_else(GetCategoryRequest::default, Query::into_inner);

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

/// Create a category.
async fn create_category(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageCategory>,
    req: Json<<category::Create as ApiEndpoint>::Req>,
) -> actix_web::Result<HttpResponse, CreateError> {
    let CreateCategoryRequest { name, parent_id } = req.into_inner();

    let (id, index) = db::category::create(&db, &name, parent_id).await?;

    Ok(HttpResponse::Created().json(NewCategoryResponse { id, index }))
}

/// Update a category.
async fn update_category(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageCategory>,
    req: Option<Json<<category::Update as ApiEndpoint>::Req>>,
    path: web::Path<CategoryId>,
) -> actix_web::Result<HttpResponse, error::CategoryUpdate> {
    let UpdateCategoryRequest {
        name,
        parent_id,
        index,
        user_scopes,
    } = req.map_or_else(Default::default, Json::into_inner);

    db::category::update(
        &db,
        path.into_inner(),
        parent_id,
        name.as_deref(),
        index.map(|it| it as i16),
        user_scopes,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a category.
async fn delete_category(
    db: Data<PgPool>,
    _claims: TokenUserWithScope<ScopeManageCategory>,
    path: web::Path<CategoryId>,
) -> actix_web::Result<HttpResponse, error::Delete> {
    db::category::delete(&db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
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
        category::Update::METHOD.route().to(update_category),
    )
    .route(
        category::Delete::PATH,
        category::Delete::METHOD.route().to(delete_category),
    );
}
