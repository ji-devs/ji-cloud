use crate::extractor::FirebaseId;
use chrono_tz::Tz;
use shared::{
    domain::{
        auth::RegisterRequest,
        meta::SubjectId,
        user::{OtherUser, UserProfile, UserScope},
    },
    error::auth::RegisterError,
};
use sqlx::postgres::PgDatabaseError;
use std::{convert::TryFrom, str::FromStr};
use uuid::Uuid;

pub async fn by_name(db: &sqlx::PgPool, name: &str) -> anyhow::Result<Option<OtherUser>> {
    Ok(sqlx::query_as!(
        OtherUser,
        r#"select id from "user" where username = $1"#,
        name
    )
    .fetch_optional(db)
    .await?)
}

pub async fn profile(db: &sqlx::PgPool, id: Uuid) -> anyhow::Result<Option<UserProfile>> {
    let row = sqlx::query!(
        r#"
        select id,
        firebase_id,
        username,
        email::text                                                              as "email!",
        given_name,
        family_name,
        language,
        locale,
        opt_into_edu_resources,
        over_18,
        timezone,
        created_at,
        updated_at,
        organization,
        array(select scope from user_scope where user_scope.user_id = "user".id) as "scopes!: Vec<i16>",
        array(select subject_id from user_subject where user_subject.user_id = "user".id) as "subjects!: Vec<Uuid>"
 from "user"
 where id = $1"#,
        id
    )
    .fetch_optional(db)
    .await?;

    let row = match row {
        Some(row) => row,
        None => return Ok(None),
    };

    Ok(Some(UserProfile {
        id: row.id,
        username: row.username,
        email: row.email,
        given_name: row.given_name,
        family_name: row.family_name,
        language: row.language,
        locale: row.locale,
        opt_into_edu_resources: row.opt_into_edu_resources,
        over_18: row.over_18,
        timezone: Tz::from_str(&row.timezone).map_err(|e| anyhow::anyhow!(e))?,
        scopes: row
            .scopes
            .into_iter()
            .map(UserScope::try_from)
            .collect::<Result<Vec<_>, ()>>()
            .map_err(|_| anyhow::anyhow!("invalid scope"))?,
        created_at: row.created_at,
        updated_at: row.updated_at,
        organization: row.organization,
        subjects: row.subjects.into_iter().map(SubjectId).collect(),
    }))
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
    (firebase_id, username, email, over_18, given_name, family_name, language, locale, timezone, opt_into_edu_resources, organization) 
VALUES 
    ($1, $2, $3::text, $4, $5, $6, $7, $8, $9, $10, $11)
returning id
        "#,
        id,
        &req.username,
        &req.email,
        req.over_18,
        &req.given_name,
        &req.family_name,
        &req.language,
        &req.locale,
        req.timezone.name(),
        req.opt_into_edu_resources,
        &req.organization,
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

        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("user_username_key") =>
        {
            RegisterError::TakenUsername
        }

        // fixme: This doesn't actually trigger right now because emails aren't marked `unique`
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint() == Some("user_email_key") =>
        {
            RegisterError::TakenEmail
        }

        e => e.into(),
    })
}
