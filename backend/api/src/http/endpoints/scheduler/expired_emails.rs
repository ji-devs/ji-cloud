use crate::token::SessionMask;
use sqlx::PgPool;

pub(crate) async fn delete_expired_emails(db: &PgPool) -> anyhow::Result<()> {
    log::debug!("reached delete expired emails");

    sqlx::query!(
        r#"
        delete from "user" 
        using session
        where "user".id = session.user_id 
        and (scope_mask & $1) = $1 
        and expires_at <= now() 
     "#,
        SessionMask::VERIFY_EMAIL.bits(),
    )
    .execute(db)
    .await?;

    Ok(())
}
