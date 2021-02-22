use crate::error;
use chrono_tz::Tz;
use shared::{
    domain::{
        auth::RegisterRequest,
        meta::{AffiliationId, AgeRangeId, SubjectId},
        user::{OtherUser, UserProfile, UserScope},
    },
    error::auth::RegisterErrorKind,
};
use sqlx::{postgres::PgDatabaseError, PgConnection};
use std::{convert::TryFrom, str::FromStr};
use uuid::Uuid;

use super::{nul_if_empty, recycle_metadata};

pub async fn lookup(
    db: &sqlx::PgPool,
    id: Option<Uuid>,
    name: Option<&str>,
) -> anyhow::Result<Option<OtherUser>> {
    Ok(sqlx::query_as!(
        OtherUser,
        r#"select user_id as "id" from user_profile where (user_id = $1 and $1 is not null) or (username = $2 and $2 is not null)"#,
        id,
        name
    )
    .fetch_optional(db)
    .await?)
}

pub async fn profile(db: &sqlx::PgPool, id: Uuid) -> anyhow::Result<Option<UserProfile>> {
    let row = sqlx::query!(
        r#"
select user_id as "id",
    username,
    user_email.email::text                                                              as "email!",
    given_name,
    family_name,
    language,
    locale,
    opt_into_edu_resources,
    over_18,
    timezone,
    user_profile.created_at,
    user_profile.updated_at,
    organization,
    location,
    array(select scope from user_scope where user_scope.user_id = "user".id) as "scopes!: Vec<i16>",
    array(select subject_id from user_subject where user_subject.user_id = "user".id) as "subjects!: Vec<Uuid>",
    array(select affiliation_id from user_affiliation where user_affiliation.user_id = "user".id) as "affiliations!: Vec<Uuid>",
    array(select age_range_id from user_age_range where user_age_range.user_id = "user".id) as "age_ranges!: Vec<Uuid>"
from "user"
inner join user_profile on "user".id = user_profile.user_id
inner join user_email using(user_id)
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
            .collect::<Result<Vec<_>, _>>()?,
        created_at: row.created_at,
        updated_at: row.updated_at,
        organization: row.organization,
        location: row.location,
        subjects: row.subjects.into_iter().map(SubjectId).collect(),
        age_ranges: row.age_ranges.into_iter().map(AgeRangeId).collect(),
        affiliations: row.affiliations.into_iter().map(AffiliationId).collect(),
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

pub async fn register_google_oauth(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    google_id: &str,
    user_id: Uuid,
) -> sqlx::Result<()> {
    sqlx::query!(
        "insert into user_auth_google (user_id, google_id) values ($1, $2)",
        user_id,
        google_id
    )
    .execute(txn)
    .await?;

    Ok(())
}

pub async fn register(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    req: &RegisterRequest,
    email: &str,
) -> Result<Uuid, error::Register> {
    let user_id = sqlx::query!(
        r#"
INSERT INTO "user" 
    (username, email, over_18, given_name, family_name, language, locale, timezone, opt_into_edu_resources, organization, location) 
VALUES 
    ($1, $2::text, $3, $4, $5, $6, $7, $8, $9, $10, $11)
returning id
        "#,
        &req.username,
        &email,
        req.over_18,
        &req.given_name,
        &req.family_name,
        &req.language,
        &req.locale,
        req.timezone.name(),
        req.opt_into_edu_resources,
        req.organization.as_deref(),
        req.location.as_ref(),
    )
    .fetch_one(&mut *txn)
    .await
    .map(|it| it.id)
    .map_err(|err| match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("user_username_key") =>
        {
            error::Register::RegisterError(RegisterErrorKind::TakenUsername)
        }

        // fixme: This doesn't actually trigger right now because emails aren't marked `unique`
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint() == Some("user_email_key") =>
        {
            error::Register::RegisterError(RegisterErrorKind::TakenEmail)
        }

        e => e.into(),
    })?;

    update_metadata(
        txn,
        user_id,
        nul_if_empty(&req.subjects),
        nul_if_empty(&req.affiliations),
        nul_if_empty(&req.age_ranges),
    )
    .await?;

    Ok(user_id)
}

pub async fn update_metadata(
    conn: &mut PgConnection,
    user_id: Uuid,
    subjects: Option<&[SubjectId]>,
    affiliations: Option<&[AffiliationId]>,
    age_ranges: Option<&[AgeRangeId]>,
) -> sqlx::Result<()> {
    const TABLE: &str = r#"user"#;

    if let Some(affiliations) = affiliations {
        recycle_metadata(&mut *conn, TABLE, user_id, affiliations).await?;
    }

    if let Some(age_ranges) = age_ranges {
        recycle_metadata(&mut *conn, TABLE, user_id, age_ranges).await?;
    }

    if let Some(subjects) = subjects {
        recycle_metadata(&mut *conn, TABLE, user_id, subjects).await?;
    }

    Ok(())
}
