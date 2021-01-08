use crate::{
    db,
    error::{self, BasicError},
    extractor::AuthUserWithScope,
    extractor::ScopeManageCategory,
    extractor::WrapAuthClaimsNoDb,
};
use paperclip::actix::{
    api_v2_errors, api_v2_operation,
    web::{self, Data, Json, Query, ServiceConfig},
    NoContent,
};
use shared::api::endpoints::{category, ApiEndpoint};
use shared::domain::category::{
    CategoryId, CategoryResponse, CategoryTreeScope, CreateCategoryRequest, GetCategoryRequest,
    NewCategoryResponse, UpdateCategoryRequest,
};
use sqlx::PgPool;

#[api_v2_errors(
    code = 400,
    code = 401,
    code = 403,
    code = 404,
    description = "Not Found: Parent Category Not Found",
    code = 500
)]
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
            Self::ParentCategoryNotFound => BasicError::with_message(
                http::StatusCode::NOT_FOUND,
                "Parent Category Not Found".to_owned(),
            )
            .into(),
            Self::InternalServerError(e) => crate::error::ise(e),
        }
    }
}

/// Get a tree of categories.
#[api_v2_operation]
async fn get_categories(
    db: Data<PgPool>,
    _claims: WrapAuthClaimsNoDb,
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
#[api_v2_operation]
async fn create_category(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageCategory>,
    req: Json<<category::Create as ApiEndpoint>::Req>,
) -> actix_web::Result<Json<<category::Create as ApiEndpoint>::Res>, CreateError> {
    let CreateCategoryRequest { name, parent_id } = req.into_inner();

    let (id, index) = db::category::create(&db, &name, parent_id).await?;

    Ok(Json(NewCategoryResponse { id, index }))
}

/// Update a category.
#[api_v2_operation]
async fn update_category(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageCategory>,
    req: Option<Json<<category::Update as ApiEndpoint>::Req>>,
    path: web::Path<CategoryId>,
) -> actix_web::Result<NoContent, error::CategoryUpdate> {
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

    Ok(NoContent)
}

/// Delete a category.
#[api_v2_operation]
async fn delete_category(
    db: Data<PgPool>,
    _claims: AuthUserWithScope<ScopeManageCategory>,
    path: web::Path<CategoryId>,
) -> actix_web::Result<NoContent, error::Delete> {
    db::category::delete(&db, path.into_inner()).await?;

    Ok(NoContent)
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
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
