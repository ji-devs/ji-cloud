use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use futures::try_join;
use shared::{
    api::{endpoints::resource, ApiEndpoint, PathParts},
    domain::{
        asset::{DraftOrLive, PrivacyLevel, UserOrMe},
        resource::{
            ResourceBrowseResponse, ResourceCountResponse, ResourceCreateRequest, ResourceId,
            ResourceLikedResponse, ResourceSearchResponse,
        },
        user::UserId,
        CreateResponse,
    },
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, resource::CreateResourceError},
    error::{self, ServiceKind},
    extractor::{ScopeAdmin, TokenUser, TokenUserWithScope},
    service::ServiceData,
};

pub mod curation;
pub mod report;

const DEFAULT_PAGE_LIMIT: u32 = 20;
const MAX_PAGE_LIMIT: u32 = 100;

/// Create a resource.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<resource::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<resource::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    println!("here");

    let db = db.as_ref();

    println!("here");

    let creator_id = auth.user_id();

    db::resource::authz(db, creator_id, None).await?;

    let req = req.map_or_else(ResourceCreateRequest::default, Json::into_inner);

    let id = db::resource::create(
        &*db,
        &req.display_name,
        &req.categories,
        &req.age_ranges,
        &req.affiliations,
        creator_id,
        &req.language,
        &req.description,
    )
    .await
    .map_err(|e| match e {
        CreateResourceError::DefaultModules(e) => {
            error::CreateWithMetadata::InternalServerError(e.into())
        }
        CreateResourceError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreateResourceError::InternalServerError(e) => {
            error::CreateWithMetadata::InternalServerError(e.into())
        }
    })?;

    Ok((
        Json(CreateResponse { id }),
        actix_web::http::StatusCode::CREATED,
    ))
}

#[instrument(skip_all)]
async fn get_live(
    db: Data<PgPool>,
    path: web::Path<ResourceId>,
) -> Result<Json<<resource::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let resource_response = db::resource::get_one(&db, path.into_inner(), DraftOrLive::Live)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(resource_response))
}

async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<ResourceId>,
) -> Result<Json<<resource::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let resource_response = db::resource::get_one(&db, path.into_inner(), DraftOrLive::Draft)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(resource_response))
}

/// Update a Resource's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<resource::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<ResourceId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;
    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::resource::update_draft(
        &*db,
        api_key,
        id,
        req.display_name.as_deref(),
        req.categories.as_deref(),
        req.age_ranges.as_deref(),
        req.affiliations.as_deref(),
        req.language.as_deref(),
        req.description.as_deref(),
        req.privacy_level.to_owned(),
        req.other_keywords.to_owned(),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a resource.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<ResourceId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, Some(id)).await?;

    db::resource::delete(&*db, id).await?;

    algolia.delete_resource(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<resource::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<resource::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, privacy_level, blocked) = auth_claims(
        db.as_ref(),
        claims,
        query.author_id,
        query.privacy_level,
        query.blocked,
    )
    .await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let resource_types = filters_for_ids_or(&query.resource_types[..]);

    let browse_future = db::resource::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        blocked,
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
        query.order_by,
    );

    let total_count_future = db::resource::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        blocked,
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (resources, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(ResourceBrowseResponse {
        resources,
        pages,
        total_resource_count: total_count,
    }))
}

fn filters_for_ids_or<T: Into<Uuid> + Copy>(ids: &[T]) -> Vec<Uuid> {
    let mut vect: Vec<Uuid> = vec![];
    for id in ids.iter().copied() {
        let id: Uuid = id.into();
        vect.push(id);
    }

    vect
}

/// Copies the contents of the draft resource data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    resource_id: Path<ResourceId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let resource_id = resource_id.into_inner();

    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, Some(resource_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) = db::resource::get_draft_and_live_ids(&mut *txn, resource_id)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_live_id = db::resource::clone_data(&mut txn, &draft_id, DraftOrLive::Live).await?;

    sqlx::query!(
        //language=SQL
        "update resource set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        resource_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live resource_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from resource_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    log::info!("AOSIJDOAIJSD");

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Clone a resource
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<ResourceId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, None).await?;

    let id = db::resource::clone_resource(db.as_ref(), parent.into_inner(), user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Search for resources.
#[instrument(skip_all)]
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<resource::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<resource::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (author_id, privacy_level, blocked) = auth_claims(
        &*db,
        claims,
        query.author_id,
        query.privacy_level,
        query.blocked,
    )
    .await?;

    let (ids, pages, total_hits) = algolia
        .search_resource(
            &query.q,
            query.page,
            query.language,
            &query.age_ranges,
            &query.affiliations,
            &query.resource_types,
            &query.categories,
            author_id,
            query.author_name,
            query.other_keywords,
            query.translated_keywords,
            &privacy_level,
            page_limit,
            blocked,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let resources: Vec<_> = db::resource::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live).await?;

    Ok(Json(ResourceSearchResponse {
        resources,
        pages,
        total_resource_count: total_hits,
    }))
}

/// Update a Resource's admin data.
async fn update_admin_data(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    req: Option<Json<<resource::ResourceAdminDataUpdate as ApiEndpoint>::Req>>,
    path: web::Path<ResourceId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::resource::update_admin_data(&*db, id, req.rating, req.blocked, req.curated)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn count(
    db: Data<PgPool>,
) -> Result<Json<<resource::Count as ApiEndpoint>::Res>, error::Server> {
    let total_count: u64 = db::resource::count(&*db, PrivacyLevel::Public).await?;

    Ok(Json(ResourceCountResponse { total_count }))
}

/// Add a like to a resource
async fn like(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<ResourceId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::resource::resource_like(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Whether a user has liked a Resource
async fn liked(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<ResourceId>,
) -> Result<Json<<resource::Liked as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.user_id();

    let is_liked = db::resource::resource_is_liked(&*db, user_id, path.into_inner()).await?;

    Ok(Json(ResourceLikedResponse { is_liked }))
}

/// Unlike to a resource
async fn unlike(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<ResourceId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::resource::resource_unlike(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Add a play to a resource
async fn view(
    db: Data<PgPool>,
    path: web::Path<ResourceId>,
) -> Result<HttpResponse, error::NotFound> {
    db::resource::resource_view(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument]
pub(crate) async fn page_limit(page_limit: Option<u32>) -> anyhow::Result<u32> {
    if let Some(limit) = page_limit {
        match limit > 0 && limit <= MAX_PAGE_LIMIT {
            true => Ok(limit),
            false => Err(anyhow::anyhow!("Page limit should be within 1-100")),
        }
    } else {
        Ok(DEFAULT_PAGE_LIMIT)
    }
}

#[instrument(skip(db, claims))]
async fn auth_claims(
    db: &PgPool,
    claims: Option<TokenUser>,
    author_id: Option<UserOrMe>,
    privacy_level: Vec<PrivacyLevel>,
    blocked: Option<bool>,
) -> Result<(Option<UserId>, Vec<PrivacyLevel>, Option<bool>), error::Auth> {
    if claims.is_none() && author_id == Some(UserOrMe::Me) {
        return Err(error::Auth::Forbidden);
    };

    if let Some(user) = claims {
        let is_admin = db::resource::is_admin(&*db, UserId(user.0.user_id)).await?;
        let user_id = user.user_id();

        if let Some(author) = author_id {
            let (author_id, privacy, blocked) = match author {
                UserOrMe::Me => (Some(user_id), privacy_level, blocked),
                UserOrMe::User(id) => {
                    let user_id = UserId(id);

                    if is_admin {
                        let block = if let Some(block) = blocked {
                            Some(block)
                        } else {
                            None
                        };

                        (Some(user_id), privacy_level, block)
                    } else {
                        (Some(user_id), vec![PrivacyLevel::Public], Some(false))
                    }
                }
            };
            return Ok((author_id, privacy, blocked));
        } else {
            if is_admin {
                return Ok((None, privacy_level, None));
            } else {
                return Ok((None, vec![PrivacyLevel::Public], Some(false)));
            }
        };
    } else {
        let author_id = author_id.map(|it| match it {
            UserOrMe::Me => None,
            UserOrMe::User(id) => Some(UserId(id)),
        });

        if let Some(id) = author_id {
            return Ok((id, vec![PrivacyLevel::Public], Some(false)));
        } else {
            return Ok((None, vec![PrivacyLevel::Public], Some(false)));
        }
    };
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <resource::Create as ApiEndpoint>::Path::PATH,
        resource::Create::METHOD.route().to(create),
    )
    .route(
        <resource::GetLive as ApiEndpoint>::Path::PATH,
        resource::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <resource::GetDraft as ApiEndpoint>::Path::PATH,
        resource::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <resource::Publish as ApiEndpoint>::Path::PATH,
        resource::Publish::METHOD.route().to(publish_draft_to_live),
    )
    .route(
        <resource::Clone as ApiEndpoint>::Path::PATH,
        resource::Clone::METHOD.route().to(clone),
    )
    .route(
        <resource::Browse as ApiEndpoint>::Path::PATH,
        resource::Browse::METHOD.route().to(browse),
    )
    .route(
        <resource::Search as ApiEndpoint>::Path::PATH,
        resource::Search::METHOD.route().to(search),
    )
    .route(
        <resource::UpdateDraftData as ApiEndpoint>::Path::PATH,
        resource::UpdateDraftData::METHOD.route().to(update_draft),
    )
    .route(
        <resource::Delete as ApiEndpoint>::Path::PATH,
        resource::Delete::METHOD.route().to(delete),
    )
    .route(
        <resource::ResourceAdminDataUpdate as ApiEndpoint>::Path::PATH,
        resource::ResourceAdminDataUpdate::METHOD
            .route()
            .to(update_admin_data),
    )
    .route(
        <resource::Unlike as ApiEndpoint>::Path::PATH,
        resource::Unlike::METHOD.route().to(unlike),
    )
    .route(
        <resource::Count as ApiEndpoint>::Path::PATH,
        resource::Count::METHOD.route().to(count),
    )
    .route(
        <resource::View as ApiEndpoint>::Path::PATH,
        resource::View::METHOD.route().to(view),
    )
    .route(
        <resource::Like as ApiEndpoint>::Path::PATH,
        resource::Like::METHOD.route().to(like),
    )
    .route(
        <resource::Liked as ApiEndpoint>::Path::PATH,
        resource::Liked::METHOD.route().to(liked),
    )
    .route(
        <resource::Unlike as ApiEndpoint>::Path::PATH,
        resource::Unlike::METHOD.route().to(unlike),
    );
}
