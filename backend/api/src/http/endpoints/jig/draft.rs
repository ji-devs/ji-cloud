use actix_web::web::{Data, Json, Path};
use shared::{
    api::{endpoints::jig, ApiEndpoint},
    domain::jig::{JigId, JigIdResponse},
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Get the id for the draft of a published jig
pub(super) async fn get_draft(
    db: Data<PgPool>,
    claims: TokenUser,
    live_id: Path<JigId>,
) -> Result<Json<<jig::draft::GetDraft as ApiEndpoint>::Res>, error::JigCloneDraft> {
    let live_id = live_id.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(live_id)).await?;

    let id = db::jig::get_draft(db.as_ref(), live_id).await?;

    Ok(Json(JigIdResponse { id }))
}

/// Get the id for the dual jig of the
pub(super) async fn get_live(
    db: Data<PgPool>,
    claims: TokenUser,
    draft_id: Path<JigId>,
) -> Result<Json<<jig::draft::GetLive as ApiEndpoint>::Res>, error::JigCloneDraft> {
    let draft_id = draft_id.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(draft_id)).await?;

    let id = db::jig::get_live(db.as_ref(), draft_id).await?;

    Ok(Json(JigIdResponse { id }))
}

/// Copies the contents of the draft jig to the live version
pub(super) async fn publish_draft_to_live(
    db: Data<PgPool>,
    claims: TokenUser,
    draft_id: Path<JigId>,
) -> Result<Json<<jig::draft::Publish as ApiEndpoint>::Res>, error::JigCloneDraft> {
    let draft_id = draft_id.into_inner();

    db::jig::authz(&*db, claims.0.user_id, Some(draft_id)).await?;

    let mut txn = db.begin().await?;

    let live_id = db::jig::get_live(db.as_ref(), draft_id).await?;

    db::jig::clone_one(
        &mut txn,
        &draft_id,
        Some(live_id),
        &claims.0.user_id,
        false,
        false,
    )
    .await?;

    txn.commit().await?;

    Err(anyhow::anyhow!("asd").into())
}
