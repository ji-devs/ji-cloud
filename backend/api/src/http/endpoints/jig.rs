use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use futures::try_join;
use ji_core::settings::RuntimeSettings;
use shared::domain::user::UserScope;
use shared::{
    api::{endpoints::jig, ApiEndpoint, PathParts},
    domain::{
        asset::{DraftOrLive, PrivacyLevel, UserOrMe},
        jig::{
            GetJigPlaylistsResponse, JigBrowseResponse, JigCountResponse, JigCreateRequest, JigId,
            JigLikedResponse, JigSearchResponse,
        },
        user::UserId,
        CreateResponse,
    },
    error::{IntoAnyhow, ServiceError, ServiceKindError},
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, jig::CreateJigError},
    error::{self},
    extractor::{get_user_id, ScopeAdmin, TokenUser, TokenUserWithScope},
    service::ServiceData,
};

mod codes;
pub mod curation;
pub mod report;

const DEFAULT_PAGE_LIMIT: u32 = 20;
const MAX_PAGE_LIMIT: u32 = 100;

/// Create a jig.
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<jig::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<jig::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    let db = db.as_ref();

    let creator_id = auth.user_id();

    db::jig::authz(db, creator_id, None).await?;

    let req = req.map_or_else(JigCreateRequest::default, Json::into_inner);

    let id = db::jig::create(
        &*db,
        &req.display_name,
        &req.categories,
        &req.age_ranges,
        &req.affiliations,
        creator_id,
        &req.language,
        &req.description,
        &req.default_player_settings,
    )
    .await
    .map_err(|e| match e {
        CreateJigError::DefaultModules(e) => {
            error::CreateWithMetadata::InternalServerError(e.into())
        }
        CreateJigError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreateJigError::InternalServerError(e) => {
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
    auth: Option<TokenUser>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = get_user_id(&auth);

    let jig_response = db::jig::get_one(&db, path.into_inner(), DraftOrLive::Live, user_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(jig_response))
}

async fn get_draft(
    db: Data<PgPool>,
    auth: Option<TokenUser>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = get_user_id(&auth);

    let jig_response = db::jig::get_one(&db, path.into_inner(), DraftOrLive::Draft, user_id)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(jig_response))
}

/// Update a JIG's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<jig::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;
    let user_id = claims.user_id();

    db::jig::authz(&*db, user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::jig::update_draft(
        &*db,
        api_key,
        id,
        req.display_name.as_deref(),
        req.categories.as_deref(),
        req.age_ranges.as_deref(),
        req.affiliations.as_deref(),
        req.language.as_deref(),
        req.description.as_deref(),
        req.default_player_settings.as_ref(),
        req.theme.as_ref(),
        req.audio_background.as_ref(),
        req.audio_effects.as_ref(),
        req.privacy_level,
        req.other_keywords,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a jig.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::jig::authz(&*db, user_id, Some(id)).await?;

    db::jig::delete(&*db, id).await?;

    algolia.delete_jig(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<jig::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, user_id, privacy_level, blocked) = auth_claims(
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

    let browse_future = db::jig::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        blocked,
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
        query.order_by,
        user_id,
    );

    let total_count_future = db::jig::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        blocked,
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (jigs, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(JigBrowseResponse {
        jigs,
        pages,
        total_jig_count: total_count,
    }))
}

#[instrument(skip(db, claims))]
async fn get_jig_playlists(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::GetJigPlaylists as ApiEndpoint>::Res>, error::Auth> {
    let jig_id = path.into_inner();
    let user_id = get_user_id(&claims);

    let playlists = db::jig::get_jig_playlists(db.as_ref(), jig_id, user_id).await?;

    Ok(Json(GetJigPlaylistsResponse { playlists }))
}

fn filters_for_ids_or<T: Into<Uuid> + Copy>(ids: &[T]) -> Vec<Uuid> {
    let mut vect: Vec<Uuid> = vec![];
    for id in ids.iter().copied() {
        let id: Uuid = id.into();
        vect.push(id);
    }

    vect
}

/// Copies the contents of the draft jig data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    jig_id: Path<JigId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let jig_id = jig_id.into_inner();

    let user_id = claims.user_id();

    db::jig::authz(&*db, user_id, Some(jig_id)).await?;

    db::jig::publish_draft_to_live(&*db, jig_id).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Clone a jig
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<JigId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let user_id = claims.user_id();

    db::jig::authz(&*db, user_id, None).await?;

    let id = db::jig::clone_jig(db.as_ref(), parent.into_inner(), user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Search for jigs.
#[instrument(skip_all)]
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<jig::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Search as ApiEndpoint>::Res>, ServiceError> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| ServiceError::InternalServerError(e.into()))?;

    let (author_id, user_id, privacy_level, blocked) = auth_claims(
        &*db,
        claims,
        query.author_id,
        query.privacy_level,
        query.blocked,
    )
    .await?;

    let (ids, pages, total_hits) = algolia
        .search_jig(
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
            query.is_rated,
        )
        .await?
        .ok_or_else(|| ServiceError::DisabledService(ServiceKindError::Algolia))?;

    let jigs: Vec<_> = db::jig::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live, user_id)
        .await
        .into_anyhow()?;

    Ok(Json(JigSearchResponse {
        jigs,
        pages,
        total_jig_count: total_hits,
    }))
}

/// Update a JIG's admin data.
async fn update_admin_data(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    req: Option<Json<<jig::JigAdminDataUpdate as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::NotFound> {
    let id = path.into_inner();

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::jig::update_admin_data(&*db, id, req)
        .await
        .map_err(|_| error::NotFound::ResourceNotFound)?;

    Ok(HttpResponse::NoContent().finish())
}

/// Admin Transfer JIGs from user id to another
async fn admin_transfer_jigs(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    req: Json<<jig::JigAdminTransfer as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::NotFound> {
    db::jig::transfer_jigs(&*db, req.from, req.to, &req.jig_ids[..])
        .await
        .map_err(|e| error::NotFound::InternalServerError(e))?;

    Ok(HttpResponse::NoContent().finish())
}

async fn count(db: Data<PgPool>) -> Result<Json<<jig::Count as ApiEndpoint>::Res>, error::Server> {
    let total_count: u64 = db::jig::count(&*db, PrivacyLevel::Public).await?;

    Ok(Json(JigCountResponse { total_count }))
}

/// Add a like to a jig
async fn like(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::jig::jig_like(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Whether a user has liked a JIG
async fn liked(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Liked as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.user_id();

    let is_liked = db::jig::jig_is_liked(&*db, user_id, path.into_inner()).await?;

    Ok(Json(JigLikedResponse { is_liked }))
}

/// Unlike to a jig
async fn unlike(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::jig::jig_unlike(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Add a play to a jig
async fn play(db: Data<PgPool>, path: web::Path<JigId>) -> Result<HttpResponse, error::NotFound> {
    db::jig::jig_play(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// remove all resources
/// NOTE: remove function after deletion of resources
pub async fn remove_resource(
    db: Data<PgPool>,
    _auth: TokenUserWithScope<ScopeAdmin>,
    _path: web::Path<JigId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let resource_ids: Vec<JigId> = db::jig::get_jig_resources(&*db).await?;

    for ids in resource_ids {
        db::jig::delete(&*db, ids).await?;

        algolia.delete_jig(ids).await;
    }

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
) -> Result<
    (
        Option<UserId>,
        Option<UserId>,
        Vec<PrivacyLevel>,
        Option<bool>,
    ),
    error::Auth,
> {
    if claims.is_none() && author_id == Some(UserOrMe::Me) {
        return Err(error::Auth::Forbidden);
    };

    if let Some(user) = claims {
        let is_admin = db::user::has_scopes(
            &*db,
            user.0.user_id,
            &[UserScope::Admin, UserScope::AdminAsset],
        )
        .await?;

        let user_id = user.user_id();

        if let Some(author) = author_id {
            let (author_id, privacy, blocked) = match author {
                UserOrMe::Me => (Some(user_id), privacy_level, blocked),
                UserOrMe::User(user_id) => {
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
            return Ok((author_id, Some(user_id), privacy, blocked));
        } else {
            if is_admin {
                return Ok((None, Some(user_id), privacy_level, None));
            } else {
                return Ok((None, Some(user_id), vec![PrivacyLevel::Public], Some(false)));
            }
        };
    } else {
        let author_id = author_id.map(|it| match it {
            UserOrMe::Me => None,
            UserOrMe::User(id) => Some(id),
        });

        if let Some(id) = author_id {
            return Ok((id, None, vec![PrivacyLevel::Public], Some(false)));
        } else {
            return Ok((None, None, vec![PrivacyLevel::Public], Some(false)));
        }
    };
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <jig::Create as ApiEndpoint>::Path::PATH,
        jig::Create::METHOD.route().to(create),
    )
    .route(
        <jig::GetLive as ApiEndpoint>::Path::PATH,
        jig::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <jig::GetDraft as ApiEndpoint>::Path::PATH,
        jig::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <jig::Publish as ApiEndpoint>::Path::PATH,
        jig::Publish::METHOD.route().to(publish_draft_to_live),
    )
    .route(
        <jig::Clone as ApiEndpoint>::Path::PATH,
        jig::Clone::METHOD.route().to(clone),
    )
    .route(
        <jig::Browse as ApiEndpoint>::Path::PATH,
        jig::Browse::METHOD.route().to(browse),
    )
    .route(
        <jig::Search as ApiEndpoint>::Path::PATH,
        jig::Search::METHOD.route().to(search),
    )
    .route(
        <jig::UpdateDraftData as ApiEndpoint>::Path::PATH,
        jig::UpdateDraftData::METHOD.route().to(update_draft),
    )
    .route(
        <jig::Delete as ApiEndpoint>::Path::PATH,
        jig::Delete::METHOD.route().to(delete),
    )
    .route(
        <jig::JigAdminDataUpdate as ApiEndpoint>::Path::PATH,
        jig::JigAdminDataUpdate::METHOD
            .route()
            .to(update_admin_data),
    )
    .route(
        <jig::JigAdminTransfer as ApiEndpoint>::Path::PATH,
        jig::JigAdminTransfer::METHOD
            .route()
            .to(admin_transfer_jigs),
    )
    .route(
        <jig::codes::Create as ApiEndpoint>::Path::PATH,
        jig::codes::Create::METHOD.route().to(codes::create),
    )
    .route(
        <jig::codes::Update as ApiEndpoint>::Path::PATH,
        jig::codes::Update::METHOD.route().to(codes::update),
    )
    .route(
        <jig::codes::JigCodeList as ApiEndpoint>::Path::PATH,
        jig::codes::JigCodeList::METHOD
            .route()
            .to(codes::list_user_codes),
    )
    .route(
        <jig::codes::JigCodeSessions as ApiEndpoint>::Path::PATH,
        jig::codes::JigCodeSessions::METHOD
            .route()
            .to(codes::list_code_sessions),
    )
    .route(
        <jig::codes::instance::Create as ApiEndpoint>::Path::PATH,
        jig::codes::instance::Create::METHOD
            .route()
            .to(codes::instance::start_session),
    )
    .route(
        <jig::codes::instance::Complete as ApiEndpoint>::Path::PATH,
        jig::codes::instance::Complete::METHOD
            .route()
            .to(codes::instance::complete_session),
    )
    .route(
        <jig::Count as ApiEndpoint>::Path::PATH,
        jig::Count::METHOD.route().to(count),
    )
    .route(
        <jig::GetJigPlaylists as ApiEndpoint>::Path::PATH,
        jig::GetJigPlaylists::METHOD.route().to(get_jig_playlists),
    )
    .route(
        <jig::Play as ApiEndpoint>::Path::PATH,
        jig::Play::METHOD.route().to(play),
    )
    .route(
        <jig::Like as ApiEndpoint>::Path::PATH,
        jig::Like::METHOD.route().to(like),
    )
    .route(
        <jig::Liked as ApiEndpoint>::Path::PATH,
        jig::Liked::METHOD.route().to(liked),
    )
    .route(
        <jig::Unlike as ApiEndpoint>::Path::PATH,
        jig::Unlike::METHOD.route().to(unlike),
    )
    .route(
        <jig::RemoveResource as ApiEndpoint>::Path::PATH,
        jig::RemoveResource::METHOD.route().to(remove_resource),
    );
}
