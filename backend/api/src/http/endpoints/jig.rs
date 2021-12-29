use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::{
        jig::{
            DeleteUserJigs, DraftOrLive, JigBrowseResponse, JigCountResponse, JigCreateRequest,
            JigId, JigLikedResponse, JigSearchResponse, PrivacyLevel, UserOrMe,
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

pub mod additional_resource;
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
        &language,
        &req.description,
        &req.default_player_settings,
        &req.jig_focus,
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

async fn get_live(
    db: Data<PgPool>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let jig_response = db::jig::get_one(&db, path.into_inner(), DraftOrLive::Live)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(jig_response))
}

async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<JigId>,
) -> Result<Json<<jig::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let jig_response = db::jig::get_one(&db, path.into_inner(), DraftOrLive::Draft)
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

    db::jig::authz(&*db, claims.0.user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::jig::update_draft(
        &*db,
        api_key,
        id,
        claims.0.user_id,
        req.display_name.as_deref(),
        req.goals.as_deref(),
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
        req.admin_data,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
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

/// Delete all jigs associated with user.
async fn delete_all(
    db: Data<PgPool>,
    claims: TokenUser,
    algolia: ServiceData<crate::algolia::Client>,
) -> Result<HttpResponse, error::Delete> {
    db::jig::authz(&*db, claims.0.user_id, None).await?;

    let id: Vec<DeleteUserJigs> = db::jig::delete_all_jigs(&*db, claims.0.user_id).await?;

    let mut ids = id.into_iter();

    loop {
        match ids.next() {
            Some(id) => match id {
                jig_id => algolia.delete_jig(jig_id.jig_id).await,
            },
            None => {
                log::warn!("Done with delete");
                break;
            }
        }
    }
    Ok(HttpResponse::NoContent().finish())
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

    println!("before browse");

    let jigs = db::jig::browse(
        db.as_ref(),
        author_id,
        query.jig_focus,
        query.page.unwrap_or(0) as i32,
    )
    .await?;

    let total_count =
        db::jig::filtered_count(db.as_ref(), None, author_id, query.jig_focus).await?;

    let pages = (total_count / 20 + (total_count % 20 != 0) as u64) as u32;

    Ok(Json(JigBrowseResponse {
        jigs,
        pages,
        total_jig_count: total_count,
    }))
}

/// Copies the contents of the draft jig data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    jig_id: Path<JigId>,
) -> Result<HttpResponse, error::JigCloneDraft> {
    let jig_id = jig_id.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(jig_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) = db::jig::get_draft_and_live_ids(&mut *txn, jig_id)
        .await
        .ok_or(error::JigCloneDraft::ResourceNotFound)?;

    let new_live_id = db::jig::clone_data(&mut txn, &draft_id).await?;

    sqlx::query!(
        //language=SQL
        "update jig set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        jig_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live jig_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from jig_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    log::info!("AOSIJDOAIJSD");

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Clone a jig
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<JigId>,
) -> Result<HttpResponse, error::JigCloneDraft> {
    db::jig::authz(&*db, claims.0.user_id, None).await?;

    let id = db::jig::clone_jig(db.as_ref(), parent.into_inner(), claims.0.user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
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
            Some(PrivacyLevel::Public),
            query.language,
            &query.age_ranges,
            &query.affiliations,
            &query.resource_types,
            &query.categories,
            &query.goals,
            query.author,
            query.author_name,
            query.jig_focus,
            query.other_keywords,
            query.translated_keywords,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let jigs: Vec<_> = db::jig::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live).await?;

    Ok(Json(JigSearchResponse {
        jigs,
        pages,
        total_jig_count: total_hits,
    }))
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
    db::jig::jig_like(&*db, claims.0.user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Whether a user has liked a JIG
async fn liked(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<jig::Liked as ApiEndpoint>::Res>, error::Server> {
    let is_liked = db::jig::jig_is_liked(&*db, claims.0.user_id, path.into_inner()).await?;

    Ok(Json(JigLikedResponse { is_liked }))
}

/// Unlike to a jig
async fn unlike(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<HttpResponse, error::Server> {
    db::jig::jig_unlike(&*db, claims.0.user_id, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Add a play to a jig
async fn play(db: Data<PgPool>, path: web::Path<JigId>) -> Result<HttpResponse, error::NotFound> {
    db::jig::jig_play(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(jig::Create::PATH, jig::Create::METHOD.route().to(create))
        .route(
            jig::GetLive::PATH,
            jig::GetLive::METHOD.route().to(get_live),
        )
        .route(
            jig::GetDraft::PATH,
            jig::GetDraft::METHOD.route().to(get_draft),
        )
        .route(
            jig::Publish::PATH,
            jig::Publish::METHOD.route().to(publish_draft_to_live),
        )
        .route(jig::Clone::PATH, jig::Clone::METHOD.route().to(clone))
        .route(jig::Browse::PATH, jig::Browse::METHOD.route().to(browse))
        .route(jig::Search::PATH, jig::Search::METHOD.route().to(search))
        .route(
            jig::UpdateDraftData::PATH,
            jig::UpdateDraftData::METHOD.route().to(update_draft),
        )
        .route(jig::Delete::PATH, jig::Delete::METHOD.route().to(delete))
        .route(
            jig::DeleteAll::PATH,
            jig::DeleteAll::METHOD.route().to(delete_all),
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
            jig::player::PlayCount::PATH,
            jig::player::PlayCount::METHOD
                .route()
                .to(player::get_play_count),
        )
        .route(jig::Count::PATH, jig::Count::METHOD.route().to(count))
        .route(jig::Play::PATH, jig::Play::METHOD.route().to(play))
        .route(jig::Like::PATH, jig::Like::METHOD.route().to(like))
        .route(jig::Liked::PATH, jig::Liked::METHOD.route().to(liked))
        .route(jig::Unlike::PATH, jig::Unlike::METHOD.route().to(unlike));
}
