use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create_new(
    conn: &mut PgConnection,
    user_id: Uuid,
    token: &str,
    valid_until: Option<&DateTime<Utc>>,
    temporary: bool,
) -> sqlx::Result<()> {
    // only allow 1 temporary token for now
    if temporary {
        sqlx::query!(
            "delete from session where user_id = $1 and temporary is true",
            user_id
        )
        .execute(&mut *conn)
        .await?;
    }

    sqlx::query!(
        "insert into session (token, user_id, expires_at, temporary) values ($1, $2, $3, $4)",
        &token,
        user_id,
        valid_until,
        temporary
    )
    .execute(&mut *conn)
    .await?;
    Ok(())
}
