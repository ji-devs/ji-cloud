use paperclip::actix::{
    api_v2_operation,
    web::{Data, ServiceConfig},
    CreatedJson,
};
use shared::{
    api::{endpoints::search, ApiEndpoint},
    domain::search::CreateSearchKeyResponse,
};

use crate::{
    error::{self, ServiceKind},
    extractor::WrapAuthClaimsNoDb,
};

/// Create an Algolia search key based on the user's auth. Currently expires after 15 minutes, but that number is subject to change.
/// # Errors
/// 501: If the server doesn't have algolia enabled, or it doesn't have a key to derive for the frontend.
#[api_v2_operation]
async fn create_key(
    algolia: Data<crate::algolia::Client>,
    claims: WrapAuthClaimsNoDb,
) -> actix_web::Result<CreatedJson<<search::CreateKey as ApiEndpoint>::Res>, error::Service> {
    let key = algolia
        .generate_virtual_key(Some(claims.0.id), Some(chrono::Duration::minutes(15)))
        .ok_or(error::Service::DisabledService(ServiceKind::Algolia))?;

    Ok(CreatedJson(CreateSearchKeyResponse { key: key.0 }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        search::CreateKey::PATH,
        search::CreateKey::METHOD.route().to(create_key),
    );
}
