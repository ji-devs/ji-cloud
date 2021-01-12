use crate::{error, extractor::FirebaseId};
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
    firebase_id: Option<&str>,
    name: Option<&str>,
) -> anyhow::Result<Option<OtherUser>> {
    Ok(sqlx::query_as!(
        OtherUser,
        r#"select id from "user" where (id = $1 and $1 is not null) or (firebase_id = $2 and $2 is not null) or (username = $3 and $3 is not null)"#,
        id,
        firebase_id,
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
        location,
        array(select scope from user_scope where user_scope.user_id = "user".id) as "scopes!: Vec<i16>",
        array(select subject_id from user_subject where user_subject.user_id = "user".id) as "subjects!: Vec<Uuid>",
        array(select affiliation_id from user_affiliation where user_affiliation.user_id = "user".id) as "affiliations!: Vec<Uuid>",
        array(select age_range_id from user_age_range where user_age_range.user_id = "user".id) as "age_ranges!: Vec<Uuid>"
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
) -> Result<Uuid, error::Register> {
    let mut txn = db.begin().await?;

    let user_id = sqlx::query!(
        r#"
INSERT INTO "user" 
    (firebase_id, username, email, over_18, given_name, family_name, language, locale, timezone, opt_into_edu_resources, organization, location) 
VALUES 
    ($1, $2, $3::text, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
        req.organization.as_deref(),
        req.location.as_ref(),
    )
    .fetch_one(&mut txn)
    .await
    .map(|it| it.id)
    .map_err(|err| match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("user_firebase_id_key") =>
        {
            error::Register
            ::RegisterError(RegisterErrorKind::TakenId)
        }

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
        &mut txn,
        user_id,
        nul_if_empty(&req.subjects),
        nul_if_empty(&req.affiliations),
        nul_if_empty(&req.age_ranges),
    )
    .await?;

    txn.commit().await?;

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
