use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::token::SessionMask;

#[must_use]
fn generate_session_token() -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub async fn create(
    conn: &mut PgConnection,
    user_id: Uuid,
    valid_until: Option<&DateTime<Utc>>,
    mask: SessionMask,
    impersonator_id: Option<Uuid>,
) -> sqlx::Result<String> {
    let session = generate_session_token();
    sqlx::query!(
        "insert into session (token, user_id, impersonator_id, expires_at, scope_mask) values ($1, $2, $3, $4, $5)", 
        &session,
        user_id,
        impersonator_id,
        valid_until,
        mask.bits()
    )
    .execute(conn).await?;

    Ok(session)
}

pub async fn clear_any(
    conn: &mut PgConnection,
    user_id: Uuid,
    mask: SessionMask,
) -> sqlx::Result<()> {
    // make sure they can't use the old link anymore
    sqlx::query!(
        "delete from session where user_id = $1 and (scope_mask | $2) <> 0",
        user_id,
        mask.bits(),
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn delete(txn: &mut PgConnection, token: &str) -> sqlx::Result<()> {
    sqlx::query!("delete from session where token = $1", &token)
        .execute(txn)
        .await?;

    Ok(())
}
