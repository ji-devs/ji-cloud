use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use futures::try_join;
use ji_core::settings::RuntimeSettings;
use shared::domain::user::UserScope;
use shared::{
    api::{endpoints::playlist, ApiEndpoint, PathParts},
    domain::{
        asset::{DraftOrLive, PrivacyLevel, UserOrMe},
        playlist::{
            PlaylistBrowseResponse, PlaylistCreateRequest, PlaylistId, PlaylistLikedResponse,
            PlaylistSearchResponse,
        },
        user::UserId,
        CreateResponse,
    },
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, playlist::CreatePlaylistError},
    error::{self, ServiceKind},
    extractor::{get_user_id, TokenUser},
    service::ServiceData,
};

pub const DEFAULT_PAGE_LIMIT: u32 = 20;
pub const MAX_PAGE_LIMIT: u32 = 100;

/// Create a Playlist
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<playlist::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<playlist::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    let db = db.as_ref();
    let creator_id = auth.user_id();

    db::playlist::authz(db, creator_id, None).await?;

    let req = req.map_or_else(PlaylistCreateRequest::default, Json::into_inner);

    let id = db::playlist::create(
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
        CreatePlaylistError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreatePlaylistError::InternalServerError(e) => {
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
    path: web::Path<PlaylistId>,
) -> Result<Json<<playlist::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = get_user_id(&auth);

    let playlist_response =
        db::playlist::get_one(&db, path.into_inner(), DraftOrLive::Live, user_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(playlist_response))
}

async fn get_draft(
    db: Data<PgPool>,
    auth: Option<TokenUser>,
    path: web::Path<PlaylistId>,
) -> Result<Json<<playlist::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let user_id = get_user_id(&auth);

    let playlist_response =
        db::playlist::get_one(&db, path.into_inner(), DraftOrLive::Draft, user_id)
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(playlist_response))
}

/// Update a Playlist's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<playlist::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<PlaylistId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;
    let user_id = claims.user_id();

    db::playlist::authz(&*db, user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::playlist::update_draft(
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

/// Delete a Playlist.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<PlaylistId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::playlist::authz(&*db, user_id, Some(id)).await?;

    db::playlist::delete(&*db, id).await?;

    algolia.delete_playlist(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<playlist::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<playlist::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, user_id, privacy_level) =
        auth_claims(db.as_ref(), claims, query.author_id, query.privacy_level).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let resource_types = filters_for_ids_or(&query.resource_types[..]);

    let browse_future = db::playlist::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
        user_id,
    );

    let total_count_future = db::playlist::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (playlists, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(PlaylistBrowseResponse {
        playlists,
        pages,
        total_playlist_count: total_count,
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

/// Copies the contents of the draft Playlist data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    playlist_id: Path<PlaylistId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let playlist_id = playlist_id.into_inner();
    let user_id = claims.user_id();

    db::playlist::authz(&*db, user_id, Some(playlist_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) = db::playlist::get_draft_and_live_ids(&mut *txn, playlist_id)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_live_id = db::playlist::clone_data(&mut txn, &draft_id, DraftOrLive::Live).await?;

    sqlx::query!(
        //language=SQL
        "update playlist set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        playlist_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live playlist_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from playlist_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for Playlists.
#[instrument(skip_all)]
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<playlist::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<playlist::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);
    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (author_id, user_id, privacy_level) =
        auth_claims(&*db, claims, query.author_id, query.privacy_level).await?;

    let (ids, pages, total_hits) = algolia
        .search_playlist(
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

    let playlists: Vec<_> =
        db::playlist::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live, user_id).await?;

    Ok(Json(PlaylistSearchResponse {
        playlists,
        pages,
        total_playlist_count: total_hits,
    }))
}

/// Clone a Playlist
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<PlaylistId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, None).await?;

    let id = db::playlist::clone_playlist(db.as_ref(), parent.into_inner(), user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
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
) -> Result<(Option<UserId>, Option<UserId>, Vec<PrivacyLevel>), error::Auth> {
    if claims.is_none() && author_id == Some(UserOrMe::Me) {
        return Err(error::Auth::Forbidden);
    };

    if let Some(user) = claims {
        let user_id = user.user_id();
        let is_admin =
            db::user::has_scopes(&*db, user_id, &[UserScope::Admin, UserScope::AdminAsset]).await?;

        if let Some(author) = author_id {
            let user_id = user.user_id();

            let (author_id, privacy) = match author {
                UserOrMe::Me => (Some(user_id), privacy_level),
                UserOrMe::User(id) => {
                    let user_id = UserId(id);

                    if is_admin {
                        (Some(user_id), privacy_level)
                    } else {
                        (Some(user_id), vec![PrivacyLevel::Public])
                    }
                }
            };
            return Ok((author_id, Some(user_id), privacy));
        } else {
            if is_admin {
                return Ok((None, Some(user_id), privacy_level));
            } else {
                return Ok((None, Some(user_id), vec![PrivacyLevel::Public]));
            }
        };
    } else {
        let author_id = author_id.map(|it| match it {
            UserOrMe::Me => None,
            UserOrMe::User(id) => Some(UserId(id)),
        });

        if let Some(id) = author_id {
            return Ok((id, None, vec![PrivacyLevel::Public]));
        } else {
            return Ok((None, None, vec![PrivacyLevel::Public]));
        }
    };
}

/// Add a like to a playlist
async fn like(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<PlaylistId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::playlist::playlist_like(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Whether a user has liked a Playlist
async fn liked(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<PlaylistId>,
) -> Result<Json<<playlist::Liked as ApiEndpoint>::Res>, error::Server> {
    let user_id = claims.user_id();

    let is_liked = db::playlist::playlist_is_liked(&*db, user_id, path.into_inner()).await?;

    Ok(Json(PlaylistLikedResponse { is_liked }))
}

/// Unlike to a playlist
async fn unlike(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<PlaylistId>,
) -> Result<HttpResponse, error::Server> {
    let user_id = claims.user_id();

    db::playlist::playlist_unlike(&*db, user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Add a play to a playlist
async fn view(
    db: Data<PgPool>,
    path: web::Path<PlaylistId>,
) -> Result<HttpResponse, error::NotFound> {
    db::playlist::playlist_play(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        <playlist::Create as ApiEndpoint>::Path::PATH,
        playlist::Create::METHOD.route().to(create),
    )
    .route(
        <playlist::GetLive as ApiEndpoint>::Path::PATH,
        playlist::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <playlist::Like as ApiEndpoint>::Path::PATH,
        playlist::Like::METHOD.route().to(like),
    )
    .route(
        <playlist::GetDraft as ApiEndpoint>::Path::PATH,
        playlist::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <playlist::Clone as ApiEndpoint>::Path::PATH,
        playlist::Clone::METHOD.route().to(clone),
    )
    .route(
        <playlist::Publish as ApiEndpoint>::Path::PATH,
        playlist::Publish::METHOD.route().to(publish_draft_to_live),
    )
    .route(
        <playlist::Browse as ApiEndpoint>::Path::PATH,
        playlist::Browse::METHOD.route().to(browse),
    )
    .route(
        <playlist::Search as ApiEndpoint>::Path::PATH,
        playlist::Search::METHOD.route().to(search),
    )
    .route(
        <playlist::UpdateDraftData as ApiEndpoint>::Path::PATH,
        playlist::UpdateDraftData::METHOD.route().to(update_draft),
    )
    .route(
        <playlist::Delete as ApiEndpoint>::Path::PATH,
        playlist::Delete::METHOD.route().to(delete),
    )
    .route(
        <playlist::View as ApiEndpoint>::Path::PATH,
        playlist::View::METHOD.route().to(view),
    )
    .route(
        <playlist::Liked as ApiEndpoint>::Path::PATH,
        playlist::Liked::METHOD.route().to(liked),
    )
    .route(
        <playlist::Unlike as ApiEndpoint>::Path::PATH,
        playlist::Unlike::METHOD.route().to(unlike),
    );
}
