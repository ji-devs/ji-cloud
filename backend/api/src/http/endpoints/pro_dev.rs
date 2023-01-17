use actix_web::{
    web::{self, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use core::settings::RuntimeSettings;
use futures::try_join;
use shared::{
    api::{endpoints::pro_dev, ApiEndpoint, PathParts},
    domain::{
        asset::{DraftOrLive, PrivacyLevel, UserOrMe},
        pro_dev::{ProDevBrowseResponse, ProDevCreateRequest, ProDevId, ProDevSearchResponse},
        user::UserId,
        CreateResponse,
    },
};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::{self, pro_dev::CreateProDevError},
    error::{self, ServiceKind},
    extractor::TokenUser,
    service::ServiceData,
};

pub mod unit;

pub const DEFAULT_PAGE_LIMIT: u32 = 20;
pub const MAX_PAGE_LIMIT: u32 = 100;

/// Create a ProDev
async fn create(
    db: Data<PgPool>,
    auth: TokenUser,
    req: Option<Json<<pro_dev::Create as ApiEndpoint>::Req>>,
) -> Result<
    (
        Json<<pro_dev::Create as ApiEndpoint>::Res>,
        actix_web::http::StatusCode,
    ),
    error::CreateWithMetadata,
> {
    let db = db.as_ref();
    let creator_id = auth.user_id();

    db::pro_dev::authz(db, creator_id, None).await?;

    let req = req.map_or_else(ProDevCreateRequest::default, Json::into_inner);

    let id = db::pro_dev::create(
        &*db,
        &req.display_name,
        &req.categories,
        creator_id,
        &req.language,
        &req.description,
    )
    .await
    .map_err(|e| match e {
        CreateProDevError::Sqlx(e) => db::meta::handle_metadata_err(e).into(),
        CreateProDevError::InternalServerError(e) => {
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
    path: web::Path<ProDevId>,
) -> Result<Json<<pro_dev::GetLive as ApiEndpoint>::Res>, error::NotFound> {
    let pro_dev_response = db::pro_dev::get_one(&db, path.into_inner(), DraftOrLive::Live)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(pro_dev_response))
}

async fn get_draft(
    db: Data<PgPool>,
    path: web::Path<ProDevId>,
) -> Result<Json<<pro_dev::GetDraft as ApiEndpoint>::Res>, error::NotFound> {
    let pro_dev_response = db::pro_dev::get_one(&db, path.into_inner(), DraftOrLive::Draft)
        .await?
        .ok_or(error::NotFound::ResourceNotFound)?;

    Ok(Json(pro_dev_response))
}

/// Update a ProDev's draft data.
async fn update_draft(
    db: Data<PgPool>,
    settings: Data<RuntimeSettings>,
    claims: TokenUser,
    req: Option<Json<<pro_dev::UpdateDraftData as ApiEndpoint>::Req>>,
    path: web::Path<ProDevId>,
) -> Result<HttpResponse, error::UpdateWithMetadata> {
    let id = path.into_inner();
    let api_key = &settings.google_api_key;
    let user_id = claims.user_id();

    db::pro_dev::authz(&*db, user_id, Some(id)).await?;

    let req = req.map_or_else(Default::default, Json::into_inner);

    db::pro_dev::update_draft(
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

/// Delete a ProDev.
async fn delete(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<ProDevId>,
    algolia: ServiceData<crate::algolia::Manager>,
) -> Result<HttpResponse, error::Delete> {
    let id = path.into_inner();
    let user_id = claims.user_id();

    db::pro_dev::authz(&*db, user_id, Some(id)).await?;

    db::pro_dev::delete(&*db, id).await?;

    // algolia.delete_pro_dev(id).await;

    Ok(HttpResponse::NoContent().finish())
}

#[instrument(skip(db, claims))]
async fn browse(
    db: Data<PgPool>,
    claims: Option<TokenUser>,
    query: Option<Query<<pro_dev::Browse as ApiEndpoint>::Req>>,
) -> Result<Json<<pro_dev::Browse as ApiEndpoint>::Res>, error::Auth> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let (author_id, privacy_level) =
        auth_claims(db.as_ref(), claims, query.author_id, query.privacy_level).await?;

    let page_limit = page_limit(query.page_limit)
        .await
        .map_err(|e| error::Auth::InternalServerError(e))?;

    let resource_types = filters_for_ids_or(&query.resource_types[..]);

    let browse_future = db::pro_dev::browse(
        db.as_ref(),
        author_id,
        query.draft_or_live,
        privacy_level.to_owned(),
        query.page.unwrap_or(0) as i32,
        page_limit,
        resource_types.to_owned(),
    );

    let total_count_future = db::pro_dev::filtered_count(
        db.as_ref(),
        privacy_level.to_owned(),
        author_id,
        query.draft_or_live,
        resource_types.to_owned(),
    );

    let (pro_devs, (total_count, count)) = try_join!(browse_future, total_count_future,)?;

    let pages = (count / (page_limit as u64) + (count % (page_limit as u64) != 0) as u64) as u32;

    Ok(Json(ProDevBrowseResponse {
        pro_devs,
        pages,
        total_pro_dev_count: total_count,
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

/// Copies the contents of the draft ProDev data to live
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    pro_dev_id: Path<ProDevId>,
) -> Result<HttpResponse, error::CloneDraft> {
    let pro_dev_id = pro_dev_id.into_inner();
    let user_id = claims.user_id();

    db::pro_dev::authz(&*db, user_id, Some(pro_dev_id)).await?;

    let mut txn = db.begin().await?;

    let (draft_id, live_id) = db::pro_dev::get_draft_and_live_ids(&mut *txn, pro_dev_id)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_live_id = db::pro_dev::clone_data(&mut txn, &draft_id, DraftOrLive::Live).await?;

    sqlx::query!(
        //language=SQL
        "update pro_dev set live_id = $1, published_at = now() where id = $2",
        new_live_id,
        pro_dev_id.0
    )
    .execute(&mut *txn)
    .await?;

    // should drop all the entries in the metadata tables that FK to the live pro_dev_data row
    sqlx::query!(
        //language=SQL
        r#"
delete from pro_dev_data where id = $1
    "#,
        live_id,
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

/// Search for ProDevs.
// #[instrument(skip_all)]
// async fn search(
//     db: Data<PgPool>,
//     claims: Option<TokenUser>,
//     algolia: ServiceData<crate::algolia::Client>,
//     query: Option<Query<<pro_dev::Search as ApiEndpoint>::Req>>,
// ) -> Result<Json<<pro_dev::Search as ApiEndpoint>::Res>, error::Service> {
//     let query = query.map_or_else(Default::default, Query::into_inner);
//     let page_limit = page_limit(query.page_limit)
//         .await
//         .map_err(|e| error::Service::InternalServerError(e))?;

//     let (author_id, privacy_level) =
//         auth_claims(&*db, claims, query.author_id, query.privacy_level).await?;

//     let (ids, pages, total_hits) = algolia
//         .search_pro_dev(
//             &query.q,
//             query.page,
//             query.language,
//             &query.resource_types,
//             &query.categories,
//             &query.units,
//             author_id,
//             query.author_name,
//             query.other_keywords,
//             query.translated_keywords,
//             &privacy_level,
//             page_limit,
//         )
//         .await?
//         .ok_or_else(|| error::Service::DisabledService(ServiceKind::Algolia))?;

//     let pro_devs: Vec<_> = db::pro_dev::get_by_ids(db.as_ref(), &ids, DraftOrLive::Live).await?;

//     Ok(Json(ProDevSearchResponse {
//         pro_devs,
//         pages,
//         total_pro_dev_count: total_hits,
//     }))
// }

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
        let is_admin = db::pro_dev::is_admin(&*db, user_id).await?;

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
        <pro_dev::Create as ApiEndpoint>::Path::PATH,
        pro_dev::Create::METHOD.route().to(create),
    )
    .route(
        <pro_dev::GetLive as ApiEndpoint>::Path::PATH,
        pro_dev::GetLive::METHOD.route().to(get_live),
    )
    .route(
        <pro_dev::GetDraft as ApiEndpoint>::Path::PATH,
        pro_dev::GetDraft::METHOD.route().to(get_draft),
    )
    .route(
        <pro_dev::Publish as ApiEndpoint>::Path::PATH,
        pro_dev::Publish::METHOD.route().to(publish_draft_to_live),
    )
    .route(
        <pro_dev::Browse as ApiEndpoint>::Path::PATH,
        pro_dev::Browse::METHOD.route().to(browse),
    )
    // .route(
    //     <pro_dev::Search as ApiEndpoint>::Path::PATH,
    //     pro_dev::Search::METHOD.route().to(search),
    // )
    .route(
        <pro_dev::UpdateDraftData as ApiEndpoint>::Path::PATH,
        pro_dev::UpdateDraftData::METHOD.route().to(update_draft),
    )
    .route(
        <pro_dev::Delete as ApiEndpoint>::Path::PATH,
        pro_dev::Delete::METHOD.route().to(delete),
    );
}
