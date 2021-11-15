use actix_web::{
    http::StatusCode,
    web::{Data, Json, Query, ServiceConfig},
};
use core::settings::RuntimeSettings;
use shared::{
    api::{endpoints::search, ApiEndpoint},
    domain::search::{CreateSearchKeyResponse, WebImageSearchResponse},
};

use crate::{error, extractor::TokenUser, service::ServiceData};

/// Create an Algolia search key based on the user's auth. Currently expires after 15 minutes, but that number is subject to change.
/// # Errors
/// 501: If the server doesn't have algolia enabled, or it doesn't have a key to derive for the frontend.
async fn create_key(
    algolia: ServiceData<crate::algolia::SearchKeyStore>,
    claims: TokenUser,
) -> actix_web::Result<(Json<<search::CreateKey as ApiEndpoint>::Res>, StatusCode), error::Service> // TODO check this
{
    let key =
        algolia.generate_virtual_key(Some(claims.0.user_id), Some(chrono::Duration::minutes(15)));

    Ok((
        Json(CreateSearchKeyResponse { key: key.0 }),
        StatusCode::CREATED,
    ))
}

/// Search for images over the web.
pub async fn search_web_images(
    runtime_settings: Data<RuntimeSettings>,
    _claims: TokenUser,
    query: Option<Query<<search::WebImageSearch as ApiEndpoint>::Req>>,
) -> Result<Json<<search::WebImageSearch as ApiEndpoint>::Res>, error::Server> {
    let query = query.map_or_else(Default::default, Query::into_inner);

    let res = match &runtime_settings.bing_search_key {
        Some(key) => crate::image_search::get_images(&query.q, query.image_type, key).await?,
        None => WebImageSearchResponse { images: Vec::new() },
    };

    Ok(Json(res))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        search::CreateKey::PATH,
        search::CreateKey::METHOD.route().to(create_key),
    )
    .route(
        search::WebImageSearch::PATH,
        search::WebImageSearch::METHOD.route().to(search_web_images),
    );
}
