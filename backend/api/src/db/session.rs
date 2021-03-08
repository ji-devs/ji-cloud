use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::token::SessionMask;

#[must_use]
fn generate_session_token() -> String {
    use rand::Rng;

    let mut bytes = [0_u8; 48];
    rand::thread_rng().fill(&mut bytes[..]);
    base64::encode_config(&bytes, base64::URL_SAFE)
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
    sqlx::query!(
        "delete from session where user_id = $1 and (scope_mask | $2) <> 0",
        user_id,
        mask.bits(),
    )
    .execute(conn)
    .await?;

    Ok(())
}

/// finds a one time session and deletes it after verifying its valididity.
pub async fn get_onetime(
    txn: &mut PgConnection,
    min_mask: SessionMask,
    token: &str,
) -> sqlx::Result<Option<Uuid>> {
    let res = sqlx::query!(
        r#"delete from session where token = $1 and (scope_mask & $2) = $2 returning user_id"#,
        token,
        min_mask.bits()
    )
    .fetch_optional(txn)
    .await?;

    Ok(res.map(|it| it.user_id))
}

pub async fn delete(txn: &mut PgConnection, token: &str) -> sqlx::Result<()> {
    sqlx::query!("delete from session where token = $1", &token)
        .execute(txn)
        .await?;

    Ok(())
}
