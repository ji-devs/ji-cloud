use crate::{error, extractor::TokenUser, service::ServiceData};
use core::settings::RuntimeSettings;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Query, ServiceConfig},
    CreatedJson,
};
use shared::{
    api::{endpoints::search, ApiEndpoint},
    domain::search::{CreateSearchKeyResponse, WebImageSearchResponse},
};

/// Create an Algolia search key based on the user's auth. Currently expires after 15 minutes, but that number is subject to change.
/// # Errors
/// 501: If the server doesn't have algolia enabled, or it doesn't have a key to derive for the frontend.
#[api_v2_operation]
async fn create_key(
    algolia: ServiceData<crate::algolia::SearchKeyStore>,
    claims: TokenUser,
) -> actix_web::Result<CreatedJson<<search::CreateKey as ApiEndpoint>::Res>, error::Service> {
    let key = algolia.generate_virtual_key(Some(claims.0.sub), Some(chrono::Duration::minutes(15)));

    Ok(CreatedJson(CreateSearchKeyResponse { key: key.0 }))
}

/// Search for images over the web.
#[api_v2_operation]
pub async fn search_web_images(
    runtime_settings: Data<RuntimeSettings>,
    _claims: TokenUser,
    query: Query<<search::WebImageSearch as ApiEndpoint>::Req>,
) -> Result<Json<<search::WebImageSearch as ApiEndpoint>::Res>, error::Server> {
    let query = query.into_inner();

    // todo: handle empty queries (they're invalid in bing)

    let res = match &runtime_settings.bing_search_key {
        Some(key) => crate::image_search::get_images(&query.q, key).await?,
        None => WebImageSearchResponse { images: Vec::new() },
    };

    Ok(Json(res))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        search::CreateKey::PATH,
        search::CreateKey::METHOD.route().to(create_key),
    )
    .route(
        search::WebImageSearch::PATH,
        search::WebImageSearch::METHOD.route().to(search_web_images),
    );
}
