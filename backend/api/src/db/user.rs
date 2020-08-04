use crate::extractor::FirebaseId;
use shared::{
    auth::{RegisterError, RegisterRequest},
    user::UserProfile,
};
use sqlx::postgres::PgDatabaseError;
use uuid::Uuid;

pub async fn profile(db: &sqlx::PgPool, id: Uuid) -> sqlx::Result<Option<UserProfile>> {
    sqlx::query_as(
        r#"
select
id,
display_name,
email::text as email,
created_at,
updated_at,
array(select scope from user_scope where user_scope.user_id = "user".id) as scopes
from "user"
where id = $1"#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

pub async fn exists(db: &sqlx::PgPool, id: Uuid) -> sqlx::Result<bool> {
    sqlx::query!(
        r#"select exists (select 1 from "user" where id = $1) as "exists!""#,
        id,
    )
    .fetch_one(db)
    .await
    .map(|it| it.exists)
}

pub async fn firebase_to_id(
    db: &sqlx::PgPool,
    FirebaseId(id): &FirebaseId,
) -> sqlx::Result<Option<Uuid>> {
    sqlx::query!(r#"select id from "user" where firebase_id = $1"#, id)
        .fetch_optional(db)
        .await
        .map(|it| it.map(|it| it.id))
}

pub async fn register(
    db: &sqlx::PgPool,
    FirebaseId(id): &FirebaseId,
    req: &RegisterRequest,
) -> Result<Uuid, RegisterError> {
    sqlx::query!(
        r#"
INSERT INTO "user" 
    (firebase_id, display_name, email) 
VALUES 
    ($1, $2, $3::text)
returning id
        "#,
        id,
        &req.display_name,
        &req.email
    )
    .fetch_one(db)
    .await
    .map(|it| it.id)
    .map_err(|err| match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("user_firebase_id_key") =>
        {
            RegisterError::TakenId
        }
        // fixme: This doesn't actually trigger right now because emails aren't marked `unique`
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint() == Some("user_email_key") =>
        {
            RegisterError::TakenEmail
        }
        _ => RegisterError::InternalServerError,
    })
}
