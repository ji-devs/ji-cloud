use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use futures::try_join;
use ji_core::settings::RuntimeSettings;
use shared::domain::user::UserScope;
use shared::{
    api::{endpoints::course, ApiEndpoint, PathParts},
    domain::{
        asset::{DraftOrLive, PrivacyLevel, UserOrMe},
        course::{CourseBrowseResponse, CourseCreateRequest, CourseId, CourseSearchResponse},
        user::UserId,
        CreateResponse,
    },
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, course::CreateCourseError},
    error::{self, ServiceKind},
    extractor::TokenUser,
    service::ServiceData,
};

pub mod unit;

pub const DEFAULT_PAGE_LIMIT: u32 = 20;
pub const MAX_PAGE_LIMIT: u32 = 100;

/// Create a Course
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<course::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<course::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    let db = db.as_ref();
    let creator_id = auth.user_id();

    db::course::authz(db, creator_id, None).await?;

    let req = req.map_or_else(CourseCreateRequest::default, Json::into_inner);

    let id = db::course::create(
        &*db,
        &req.display_name,
        &req.categories,
        creator_id,
        &req.language,
        &req.description,
    )
    .await
    .map_err(|e| match e {
        CreateCourseError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreateCourseError::InternalServerError(e) => {
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
    path: web::Path<CourseId>,
) -> Result<Json<<course::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let course_response = db::course::get_one(&db, path.into_inner(), DraftOrLive::Live)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(course_response))
}

async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<CourseId>,
) -> Result<Json<<course::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let course_response = db::course::get_one(&db, path.into_inner(), DraftOrLive::Draft)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(course_response))
}

/// Update a Course's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<course::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<CourseId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;
    let user_id = claims.user_id();

    db::course::authz(&*db, user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::course::update_draft(
        &*db,
        api_key,
        id,
        req.display_name.as_deref(),
        req.categories.as_deref(),
        req.language.as_deref(),
        req.description.as_deref(),
        req.privacy_level,
        req.other_keywords,
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Delete a Course.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<CourseId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::course::authz(&*db, user_id, Some(id)).await?;

    db::course::delete(&*db, id).await?;

    algolia.delete_course(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<course::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<course::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, privacy_level) =
        auth_claims(db.as_ref(), claims, query.author_id, query.privacy_level).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let resource_types = filters_for_ids_or(&query.resource_types[..]);

    let browse_future = db::course::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
        query.order_by,
    );

    let total_count_future = db::course::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (courses, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(CourseBrowseResponse {
        courses,
        pages,
        total_course_count: total_count,
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

/// Copies the contents of the draft Course data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    course_id: Path<CourseId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let course_id = course_id.into_inner();
    let user_id = claims.user_id();

    db::course::authz(&*db, user_id, Some(course_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) = db::course::get_draft_and_live_ids(&mut *txn, course_id)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_live_id = db::course::clone_data(&mut txn, &draft_id, DraftOrLive::Live).await?;

    sqlx::query!(
        //language=SQL
        "update course set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        course_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live course_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from course_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for Courses.
#[instrument(skip_all)]
async fn search(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    algolia: ServiceData<crate::algolia::Client>,
    query: Option<Query<<course::Search as ApiEndpoint>::Req>>,
) -> Result<Json<<course::Search as ApiEndpoint>::Res>, error::Service> {
    let query = query.map_or_else(Default::default, Query::into_inner);
    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Service::InternalServerError(e))?;

    let (author_id, privacy_level) =
        auth_claims(&*db, claims, query.author_id, query.privacy_level).await?;

    let (ids, pages, total_hits) = algolia
        .search_course(
            &query.q,
            query.page,
            query.language,
            &query.resource_types,
            &query.categories,
            author_id,
            query.author_name,
            query.other_keywords,
            query.translated_keywords,
            &privacy_level,
            page_limit,
        )
        .await?
        .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

    let courses: Vec<_> = db::course::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live).await?;

    Ok(Json(CourseSearchResponse {
        courses,
        pages,
        total_course_count: total_hits,
    }))
}

/// Clone a Course
async fn clone(
    db: Data<PgPool>,
    claims: TokenUser,
    parent: web::Path<CourseId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let user_id = claims.user_id();

    db::resource::authz(&*db, user_id, None).await?;

    let id = db::course::clone_course(db.as_ref(), parent.into_inner(), user_id).await?;

    Ok(HttpResponse::Created().json(CreateResponse { id }))
}

/// Add a play to a Course
async fn play(
    db: Data<PgPool>,
    path: web::Path<CourseId>,
) -> Result<HttpResponse, error::NotFound> {
    db::course::course_play(&*db, path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
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
) -> Result<(Option<UserId>, Vec<PrivacyLevel>), error::Auth> {
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
            UserOrMe::User(id) => Some(UserId(id)),
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
        <course::Create as ApiEndpoint>::Path::PATH,
        course::Create::METHOD.route().to(create),
    )
    .route(
        <course::GetLive as ApiEndpoint>::Path::PATH,
        course::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <course::Clone as ApiEndpoint>::Path::PATH,
        course::Clone::METHOD.route().to(clone),
    )
    .route(
        <course::GetDraft as ApiEndpoint>::Path::PATH,
        course::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <course::Publish as ApiEndpoint>::Path::PATH,
        course::Publish::METHOD.route().to(publish_draft_to_live),
    )
    .route(
        <course::Browse as ApiEndpoint>::Path::PATH,
        course::Browse::METHOD.route().to(browse),
    )
    .route(
        <course::Search as ApiEndpoint>::Path::PATH,
        course::Search::METHOD.route().to(search),
    )
    .route(
        <course::Play as ApiEndpoint>::Path::PATH,
        course::Play::METHOD.route().to(play),
    )
    .route(
        <course::UpdateDraftData as ApiEndpoint>::Path::PATH,
        course::UpdateDraftData::METHOD.route().to(update_draft),
    )
    .route(
        <course::Delete as ApiEndpoint>::Path::PATH,
        course::Delete::METHOD.route().to(delete),
    );
}
