use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::token::TokenPurpose;

pub async fn create_new(
    conn: &mut PgConnection,
    user_id: Uuid,
    token: &str,
    valid_until: Option<&DateTime<Utc>>,
    purpose: Option<TokenPurpose>,
    impersonator_id: Option<Uuid>,
) -> sqlx::Result<()> {
    sqlx::query!(
        "insert into session (token, user_id, impersonator_id, expires_at, scope) values ($1, $2, $3, $4, $5)",
        &token,
        user_id,
        impersonator_id,
        valid_until,
        purpose.map(|it| it as i16)
    )
    .execute(&mut *conn)
    .await?;
    Ok(())
}
