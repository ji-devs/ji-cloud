use actix_web::{
    web::{self, Data, Json, Query, ServiceConfig},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{
            Jig, JigBrowseResponse, JigCountResponse, JigCreateRequest, JigId, JigResponse,
            JigSearchResponse, PrivacyLevel, UserOrMe,
        },
        CreateResponse,
    },
};
use sqlx::PgPool;

use crate::{
    db::{self, jig::CreateJigError},
    error::{self, ServiceKind},
    extractor::TokenUser,
    service::ServiceData,
};

mod draft;
pub mod module;
mod player;

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
    db::jig::authz(db, auth.0.user_id, None).await?;

    let req = req.map_or_else(JigCreateRequest::default, Json::into_inner);
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

    let id = db::jig::create(
        &*db,
        &req.display_name,
        &req.goals,
        &req.categories,
        &req.age_ranges,
        &req.affiliations,
        creator_id,
        req.publish_at.map(DateTime::<Utc>::from),
        &language,
        &req.description,
        &req.default_player_settings,
    )
    .await
    .map_err(|e| match e {
        CreateJigError::DefaultModules(e) => {
            error::CreateWithMetadata::InternalServerError(e.into())
        }
        CreateJigError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
    })?;

    Ok((
        Json(CreateResponse { id }),
        actix_web::http::StatusCode::CREATED,
    ))
}

/// Clone a jig
///
/// FIXME
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<JigId>,
) -> Result<HttpResponse, error::JigCloneDraft> {
    db::jig::authz(&*db, claims.0.user_id, None).await?;

    let id =
        db::jig::clone_jig_and_draft(db.as_ref(), parent.into_inner(), claims.0.user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Delete a jig.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    db::jig::delete(&*db, id).await?;

    algolia.delete_jig(id).await;

    Ok(HttpResponse::NoContent().finish())
}

/// Update a jig.
async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Option<Json<<jig::Update as ApiEndpoint>::Req>>,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::jig::update(
        &*db,
        id,
        req.display_name.as_deref(),
        req.author_id,
        req.goals.as_deref(),
        req.categories.as_deref(),
        req.age_ranges.as_deref(),
        req.affiliations.as_deref(),
        req.privacy_level.as_ref(),
        req.language.as_deref(),
        req.description.as_deref(),
        req.default_player_settings.as_ref(),
        req.theme.as_ref(),
        req.audio_background.as_ref(),
        req.audio_effects.as_ref(),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Get a jig.
async fn get(
    db: Data<PgPool>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Get as ApiEndpoint>::Res>, error::NotFound> {
    let jig = db::jig::get(&db, path.into_inner())
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(JigResponse { jig }))
}

async fn browse(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Option<Query<<jig::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let author_id = query.author_id.map(|it| match it {
        UserOrMe::Me => claims.0.user_id,
        UserOrMe::User(id) => id,
    });

    db::jig::authz_list(&*db, claims.0.user_id, author_id).await?;

    let jigs = db::jig::list(
        db.as_ref(),
        query.is_published,
        author_id,
        query.page.unwrap_or(0) as i32,
    )
    .await?;

    let total_count =
        db::jig::filtered_count(db.as_ref(), query.is_published, None, author_id).await?;

    let pages = (total_count / 20 + (total_count % 20 != 0) as u64) as u32;

    Ok(Json(JigBrowseResponse {
        jigs,
        pages,
        total_jig_count: total_count,
    }))
}

/// Search for jigs.
async fn search(
    db: Data<PgPool>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<jig::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<jig::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (ids, pages, total_hits) = algolia
        .search_jig(
            &query.q,
            query.page,
            query.is_published,
            None, // FIXME
            query.language,
            &query.age_ranges,
            &query.affiliations,
            &query.categories,
            &query.goals,
            query.author,
            query.author_name,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let jigs: Vec<_> = db::jig::get_by_ids(db.as_ref(), &ids, PrivacyLevel::Public, false)
        .await?
        .into_iter()
        .map(|jig: Jig| JigResponse { jig })
        .collect();

    Ok(Json(JigSearchResponse {
        jigs,
        pages,
        total_jig_count: total_hits,
    }))
}

async fn count(db: Data<PgPool>) -> Result<Json<<jig::Count as ApiEndpoint>::Res>, error::Server> {
    let total_count: u64 =
        db::jig::filtered_count(&*db, Some(true), Some(PrivacyLevel::Public), None).await?;

    Ok(Json(JigCountResponse { total_count }))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(jig::Browse::PATH, jig::Browse::METHOD.route().to(browse))
        .route(jig::Count::PATH, jig::Count::METHOD.route().to(count))
        .route(jig::Get::PATH, jig::Get::METHOD.route().to(get))
        .route(jig::Clone::PATH, jig::Clone::METHOD.route().to(clone))
        .route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(jig::Search::PATH, jig::Search::METHOD.route().to(search))
        .route(jig::Update::PATH, jig::Update::METHOD.route().to(update))
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete))
        .route(
            jig::player::Create::PATH,
            jig::player::Create::METHOD.route().to(player::create),
        )
        .route(
            jig::player::Create::PATH,
            jig::player::Create::METHOD.route().to(player::create),
        )
        .route(
            jig::player::List::PATH,
            jig::player::List::METHOD.route().to(player::list),
        )
        .route(
            jig::player::instance::Create::PATH,
            jig::player::instance::Create::METHOD
                .route()
                .to(player::instance::create_session_instance),
        )
        .route(
            jig::player::instance::Complete::PATH,
            jig::player::instance::Complete::METHOD
                .route()
                .to(player::instance::complete_session_instance),
        )
        .route(
            jig::draft::GetLive::PATH,
            jig::draft::GetLive::METHOD.route().to(draft::get_live),
        )
        .route(
            jig::draft::GetDraft::PATH,
            jig::draft::GetDraft::METHOD.route().to(draft::get_draft),
        )
        .route(
            jig::draft::Publish::PATH,
            jig::draft::Publish::METHOD
                .route()
                .to(draft::publish_draft_to_live),
        )
        .route(
            jig::player::PlayCount::PATH,
            jig::player::PlayCount::METHOD
                .route()
                .to(player::get_play_count),
        );
}
