use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use futures::try_join;
use shared::{
    api::{endpoints::learning_path, ApiEndpoint},
    domain::{
        jig::{DraftOrLive, PrivacyLevel, UserOrMe},
        learning_path::{
            LearningPathBrowseResponse, LearningPathCreateRequest, LearningPathId,
            LearningPathSearchResponse,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, learning_path::CreateLearningPathError},
    error::{self, ServiceKind},
    extractor::TokenUser,
    service::ServiceData,
};

const DEFAULT_PAGE_LIMIT: u32 = 20;
const MAX_PAGE_LIMIT: u32 = 100;

/// Create a Learning Path
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<learning_path::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<learning_path::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    let db = db.as_ref();

    db::learning_path::authz(db, auth.0.user_id, None).await?;

    let req = req.map_or_else(LearningPathCreateRequest::default, Json::into_inner);
    let creator_id = auth.0.user_id;

    let language = match req.language {
        Some(lang) => lang,
        None => {
            sqlx::query!(
                "select language from user_profile where user_id = $1",
                auth.0.user_id
            )
            .fetch_one(db)
            .await?
            .language
        }
    };

    let id = db::learning_path::create(
        &*db,
        &req.display_name,
        &req.categories,
        &req.age_ranges,
        &req.affiliations,
        creator_id,
        &language,
        &req.description,
    )
    .await
    .map_err(|e| match e {
        CreateLearningPathError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreateLearningPathError::InternalServerError(e) => {
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
    path: web::Path<LearningPathId>,
) -> Result<Json<<learning_path::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let learning_path_response =
        db::learning_path::get_one(&db, path.into_inner(), DraftOrLive::Live)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(learning_path_response))
}

async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<LearningPathId>,
) -> Result<Json<<learning_path::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let learning_path_response =
        db::learning_path::get_one(&db, path.into_inner(), DraftOrLive::Draft)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(learning_path_response))
}

/// Update a Learning Path's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<learning_path::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<LearningPathId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;

    db::learning_path::authz(&*db, claims.0.user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::learning_path::update_draft(
        &*db,
        api_key,
        id,
        req.display_name.as_deref(),
        req.categories.as_deref(),
        req.age_ranges.as_deref(),
        req.affiliations.as_deref(),
        req.language.as_deref(),
        req.description.as_deref(),
        req.privacy_level,
        req.other_keywords,
        req.items.as_deref(),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a Learning Path.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<LearningPathId>,
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();

    db::learning_path::authz(&*db, claims.0.user_id, Some(id)).await?;

    db::learning_path::delete(&*db, id).await?;

    algolia.delete_learning_path(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<learning_path::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<learning_path::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, privacy_level) =
        auth_claims(db.as_ref(), claims, query.author_id, query.privacy_level).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let resource_types = filters_for_ids_or(&query.resource_types[..]);

    let browse_future = db::learning_path::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
    );

    let total_count_future = db::learning_path::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (learning_paths, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(LearningPathBrowseResponse {
        learning_paths,
        pages,
        total_learning_path_count: total_count,
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

/// Copies the contents of the draft Learning Path data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    learning_path_id: Path<LearningPathId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let learning_path_id = learning_path_id.into_inner();

    db::learning_path::authz(&*db, claims.0.user_id, Some(learning_path_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) =
        db::learning_path::get_draft_and_live_ids(&mut *txn, learning_path_id)
            .await
            .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_live_id = db::learning_path::clone_data(&mut txn, &draft_id, DraftOrLive::Live).await?;

    sqlx::query!(
        //language=SQL
        "update learning_path set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        learning_path_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live learning_path_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from learning_path_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for Learning Paths.
#[instrument(skip_all)]
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<learning_path::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<learning_path::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (author_id, privacy_level) =
        auth_claims(&*db, claims, query.author_id, query.privacy_level).await?;

    let (ids, pages, total_hits) = algolia
        .search_learning_path(
            &query.q,
            query.page,
            query.language,
            &query.age_ranges,
            &query.affiliations,
            &query.resource_types,
            &query.categories,
            &query.items,
            author_id,
            query.author_name,
            query.other_keywords,
            query.translated_keywords,
            &privacy_level,
            page_limit,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let learning_paths: Vec<_> =
        db::learning_path::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live).await?;

    Ok(Json(LearningPathSearchResponse {
        learning_paths,
        pages,
        total_learning_path_count: total_hits,
    }))
}

#[instrument]
async fn page_limit(page_limit: Option<u32>) -> anyhow::Result<u32> {
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
) -> Result<(Option<Uuid>, Vec<PrivacyLevel>), error::Auth> {
    if claims.is_none() && author_id == Some(UserOrMe::Me) {
        return Err(error::Auth::Forbidden);
    };

    if let Some(user) = claims {
        let is_admin = db::learning_path::is_admin(&*db, user.0.user_id).await?;

        if let Some(author) = author_id {
            let (author_id, privacy) = match author {
                UserOrMe::Me => (Some(user.0.user_id), privacy_level),
                UserOrMe::User(id) => {
                    if is_admin {
                        (Some(id), privacy_level)
                    } else {
                        (Some(id), vec![PrivacyLevel::Public])
                    }
                }
            };
            return Ok((author_id, privacy));
        } else {
            if is_admin {
                return Ok((None, privacy_level));
            } else {
                return Ok((None, vec![PrivacyLevel::Public]));
            }
        };
    } else {
        let author_id = author_id.map(|it| match it {
            UserOrMe::Me => None,
            UserOrMe::User(id) => Some(id),
        });

        if let Some(id) = author_id {
            return Ok((id, vec![PrivacyLevel::Public]));
        } else {
            return Ok((None, vec![PrivacyLevel::Public]));
        }
    };
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        learning_path::Create::PATH,
        learning_path::Create::METHOD.route().to(create),
    )
    .route(
        learning_path::GetLive::PATH,
        learning_path::GetLive::METHOD.route().to(get_live),
    )
    .route(
        learning_path::GetDraft::PATH,
        learning_path::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        learning_path::Publish::PATH,
        learning_path::Publish::METHOD
            .route()
            .to(publish_draft_to_live),
    )
    .route(
        learning_path::Browse::PATH,
        learning_path::Browse::METHOD.route().to(browse),
    )
    .route(
        learning_path::Search::PATH,
        learning_path::Search::METHOD.route().to(search),
    )
    .route(
        learning_path::UpdateDraftData::PATH,
        learning_path::UpdateDraftData::METHOD
            .route()
            .to(update_draft),
    )
    .route(
        learning_path::Delete::PATH,
        learning_path::Delete::METHOD.route().to(delete),
    );
}
