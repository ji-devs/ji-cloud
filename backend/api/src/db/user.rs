use crate::extractor::FirebaseId;
use shared::{
    auth::{RegisterError, RegisterRequest},
    user::UserProfile,
};
use sqlx::postgres::PgDatabaseError;

pub async fn profile_by_firebase(
    db: &sqlx::PgPool,
    FirebaseId(id): &FirebaseId,
) -> sqlx::Result<Option<UserProfile>> {
    sqlx::query_as(
        r#"
select
firebase_id,
display_name,
email::text as email,
created_at,
updated_at,
array(select scope from user_scope where user_scope.user_id = "user".id) as scopes
from "user"
where firebase_id = $1"#,
    )
    .bind(id)
    .fetch_optional(db)
    .await
}

pub async fn exists_by_firebase(
    db: &sqlx::PgPool,
    FirebaseId(id): &FirebaseId,
) -> sqlx::Result<bool> {
    sqlx::query!(
        r#"select exists (select 1 from "user" where firebase_id = $1) as "exists!""#,
        id
    )
    .fetch_one(db)
    .await
    .map(|it| it.exists)
}

pub async fn register(
    db: &sqlx::PgPool,
    FirebaseId(id): &FirebaseId,
    req: &RegisterRequest,
) -> Result<(), RegisterError> {
    sqlx::query!(
        r#"
INSERT INTO "user" 
    (firebase_id, display_name, email) 
VALUES 
    ($1, $2, $3::text)
        "#,
        id,
        &req.display_name,
        &req.email
    )
    .execute(db)
    .await
    .map(drop)
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
