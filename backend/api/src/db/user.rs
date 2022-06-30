use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use shared::domain::{
    circle::CircleId,
    image::ImageId,
    meta::{
        AffiliationId, AgeRangeId, GoogleAddressComponent, GoogleAddressType, GoogleLocation,
        SubjectId,
    },
    user::{
        CreateProfileRequest, OtherUser, PatchProfileRequest, UserProfile, UserProfileExport,
        UserScope,
    },
};
use sqlx::{PgConnection, PgPool};
use std::{convert::TryFrom, str::FromStr};
use tracing::{instrument, Instrument};
use uuid::Uuid;

pub(crate) mod public_user;

use super::{nul_if_empty, recycle_metadata};
use crate::error;

#[instrument(skip(db))]
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

#[instrument(skip(db))]
pub async fn get_profile(db: &sqlx::PgPool, id: Uuid) -> anyhow::Result<Option<UserProfile>> {
    log::warn!("in get_profile: {:?}", id);
    let row = sqlx::query!(
        //language=SQL
        r#"
select user_id as "id",
    username,
    user_email.email::text as "email!",
    given_name,
    family_name,
    profile_image_id       as "profile_image?: ImageId",
    (select case when exists(select * from user_auth_google where user_id = $1) = true then 1 else 0 end)     as "is_oauth!: bool",           
    language,
    locale,
    bio,    
    location_public, 
    language_public, 
    persona_public, 
    organization_public, 
    opt_into_edu_resources,
    over_18,
    timezone,
    user_profile.created_at,
    user_profile.updated_at,
    organization,
    persona                as "persona!: Vec<String>",
    location,
    array(select scope from user_scope where user_scope.user_id = "user".id) as "scopes!: Vec<i16>",
    array(select subject_id from user_subject where user_subject.user_id = "user".id) as "subjects!: Vec<Uuid>",
    array(select affiliation_id from user_affiliation where user_affiliation.user_id = "user".id) as "affiliations!: Vec<Uuid>",
    array(select age_range_id from user_age_range where user_age_range.user_id = "user".id) as "age_ranges!: Vec<Uuid>",
    array(select circle.id 
        from circle_member bm 
        inner join circle on bm.id = circle.id 
        where bm.user_id = "user".id or circle.creator_id = "user".id
    ) as "circles!: Vec<Uuid>"
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
        None => {
            return Ok(None);
        }
    };

    Ok(Some(UserProfile {
        id: row.id,
        username: row.username,
        email: row.email,
        is_oauth: row.is_oauth,
        given_name: row.given_name,
        family_name: row.family_name,
        profile_image: row.profile_image,
        language: row.language,
        bio: row.bio,
        location_public: row.location_public,
        organization_public: row.organization_public,
        persona_public: row.persona_public,
        language_public: row.language_public,
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
        persona: row.persona,
        location: row.location,
        subjects: row.subjects.into_iter().map(SubjectId).collect(),
        age_ranges: row.age_ranges.into_iter().map(AgeRangeId).collect(),
        affiliations: row.affiliations.into_iter().map(AffiliationId).collect(),
        circles: row.circles.into_iter().map(CircleId).collect(),
    }))
}

pub async fn user_profiles_by_date_range(
    db: &sqlx::PgPool,
    from_date: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
) -> anyhow::Result<Vec<UserProfileExport>> {
    let rows = sqlx::query!(
        //language=SQL
        r#"
select user_id              as "id!",
    username                as "username!",
    user_email.email::text  as "email!",
    given_name              as "given_name!",
    family_name             as "family_name!",
    profile_image_id        as "profile_image?: ImageId",
    language                as "language!",
    user_profile.created_at as "created_at!",
    user_profile.updated_at,
    organization,
    persona                 as "persona!: Vec<String>",
    location,
    array(
        select subject.display_name
        from subject
        inner join user_subject on subject.subject_id = user_subject.subject_id
        where user_subject.user_id = "user".id
    ) as "subjects!: Vec<String>",
    array(
        select affiliation.display_name
        from affiliation
        inner join user_affiliation on affiliation.id = user_affiliation.affiliation_id
        where user_affiliation.user_id = "user".id
    ) as "affiliations!: Vec<String>",
    array(
        select age_range.display_name
        from age_range
        inner join user_age_range on age_range.id = user_age_range.age_range_id
        where user_age_range.user_id = "user".id
    ) as "age_ranges!: Vec<String>"
from "user"
    inner join user_profile on "user".id = user_profile.user_id
    inner join user_email using(user_id)
where
    (
        user_profile.created_at >= case when $1::timestamptz is null then to_timestamp('-infinity') else $1 end
        and user_profile.created_at < case when $2::timestamptz is null then to_timestamp('infinity') else $2 end
    )
    or (
        user_profile.updated_at >= case when $1::timestamptz is null then to_timestamp('-infinity') else $1 end
        and user_profile.updated_at < case when $2::timestamptz is null then to_timestamp('infinity') else $2 end
    )
"#,
        from_date,
        to_date,
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            // TODO I'm unsure how to get the string value deserialized to
            // a serde_json::Value::Object variant in the resultset.
            // This logic gets the string representation, and converts it into an Object and then
            // finally the GoogleAddressComponent.
            let location = row.location.map_or(None, |location_str| {
                location_str.as_str().map_or(None, |location_str| {
                    serde_json::from_str(&location_str).ok()
                })
            });
            let location: Option<GoogleLocation> = location.map_or(None, |location_value| {
                serde_json::from_value(location_value).ok()
            });

            let city: Option<&GoogleAddressComponent> = location.as_ref().map_or(None, |l| {
                l.place
                    .address_component_by_type(GoogleAddressType::Locality)
            });

            let country: Option<&GoogleAddressComponent> = location.as_ref().map_or(None, |l| {
                l.place
                    .address_component_by_type(GoogleAddressType::Country)
            });

            UserProfileExport {
                id: row.id,
                username: row.username,
                email: row.email,
                given_name: row.given_name,
                family_name: row.family_name,
                profile_image: row.profile_image,
                language: row.language,
                created_at: row.created_at,
                updated_at: row.updated_at,
                organization: row.organization,
                persona: row.persona,
                city: city.map(|c| c.into()),
                country: country.map(|c| c.into()),
                subjects: row.subjects,
                age_ranges: row.age_ranges,
                affiliations: row.affiliations,
            }
        })
        .collect())
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

#[instrument(skip(txn, req))]
pub async fn upsert_profile(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    req: &CreateProfileRequest,
    profile_image_id: Option<ImageId>,
    user_id: Uuid,
) -> Result<(), error::UserUpdate> {
    sqlx::query!(
        //language=SQL
        r#"
insert into user_profile
    (user_id, username, over_18, given_name, family_name, profile_image_id, language, locale, timezone, opt_into_edu_resources, organization, persona, location)
values
    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
on conflict (user_id) do update
set
    over_18 = $3,
    given_name = $4,
    family_name = $5,
    profile_image_id = $6,
    language = $7,
    locale = $8,
    timezone = $9,
    opt_into_edu_resources = $10,
    organization = $11,
    persona = $12,
    location = $13
"#,
        user_id,
        &req.username,
        req.over_18,
        &req.given_name,
        &req.family_name,
        profile_image_id.map(|it| it.0),
        &req.language,
        &req.locale,
        req.timezone.name(),
        req.opt_into_edu_resources,
        req.organization.as_deref(),
        &req.persona,
        req.location.as_ref(),
    )
    .execute(&mut *txn)
    .instrument(tracing::info_span!("insert user_profile"))
    .await?;

    sqlx::query!(
        "insert into user_scope (user_id, scope) values ($1, $2)",
        user_id,
        UserScope::ManageSelfJig as i16
    )
    .execute(&mut *txn)
    .instrument(tracing::info_span!("insert user_scope"))
    .await?;

    update_metadata(
        txn,
        user_id,
        nul_if_empty(&req.subjects),
        nul_if_empty(&req.affiliations),
        nul_if_empty(&req.age_ranges),
    )
    .await?;

    Ok(())
}

#[instrument(skip(db, req))]
pub async fn update_profile(
    db: &PgPool,
    user_id: Uuid,
    req: PatchProfileRequest,
) -> Result<(), error::UserUpdate> {
    let mut txn = db.begin().await?;

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
    "#,
        user_id
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("user exists"))
    .await?
    .exists
    {
        return Err(error::UserUpdate::UserNotFound);
    }

    if let Some(organization) = req.organization {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set organization = $2,
    updated_at = now()
where user_id = $1 and organization is distinct from $2"#,
            user_id,
            organization
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update organization"))
        .await?;
    }

    if let Some(location) = req.location {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set location = $2,
    updated_at = now()
where user_id = $1 and location is distinct from $2"#,
            user_id,
            location
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update location"))
        .await?;
    }

    if let Some(profile_image) = req.profile_image {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set profile_image_id = $2,
    updated_at = now()
where user_id = $1 and profile_image_id is distinct from $2
        "#,
            user_id,
            profile_image.map(|it| it.0),
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update profile_image"))
        .await?;
    }

    if let Some(persona) = req.persona {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set persona = $2,
    updated_at = now()
where user_id = $1 and persona is distinct from $2
        "#,
            user_id,
            &persona
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update persona"))
        .await?;
    }

    sqlx::query!(
        //language=SQL
        r#"
update user_profile
set username               = coalesce($2, username),
    given_name             = coalesce($3, given_name),
    family_name            = coalesce($4, family_name),
    language               = coalesce($5, language),
    locale                 = coalesce($6, locale),
    timezone               = coalesce($7, timezone),
    opt_into_edu_resources = coalesce($8, opt_into_edu_resources),
    persona_public         = coalesce($9, persona_public),
    organization_public    = coalesce($10, organization_public),
    location_public         = coalesce($11, location_public),
    language_public         = coalesce($12, language_public),
    bio                    = coalesce($13, bio),
    updated_at             = coalesce(now(), updated_at)
where user_id = $1
  and (($2::text is not null and $2 is distinct from username) or
       ($3::text is not null and $3 is distinct from given_name) or
       ($4::text is not null and $4 is distinct from family_name) or
       ($5::text is not null and $5 is distinct from language) or
       ($6::text is not null and $6 is distinct from locale) or
       ($7::text is not null and $7 is distinct from timezone) or
       ($8::bool is not null and $8 is distinct from opt_into_edu_resources) or
       ($9::bool is not null and $9 is distinct from persona_public) or
       ($10::bool is not null and $10 is distinct from organization_public) or
       ($11::bool is not null and $11 is distinct from location_public) or
       ($12::bool is not null and $12 is distinct from language_public) or
       ($13::text is not null and $13 is distinct from bio)
    )
    "#,
        user_id,
        req.username,
        req.given_name,
        req.family_name,
        req.language,
        req.locale,
        req.timezone.map(|it| it.to_string()),
        req.opt_into_edu_resources,
        req.persona_public,
        req.organization_public,
        req.location_public,
        req.language_public,
        req.bio
    )
    .execute(&mut txn)
    .instrument(tracing::info_span!("update user_profile"))
    .await?;

    update_metadata(
        &mut txn,
        user_id,
        req.subjects.as_deref(),
        req.affiliations.as_deref(),
        req.age_ranges.as_deref(),
    )
    .await?;

    txn.commit().await?;

    Ok(())
}

#[instrument(skip(conn))]
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

fn rgba_to_i32(color: rgb::RGBA8) -> i32 {
    i32::from_be_bytes(color.into())
}

fn color_to_rgba(color: i32) -> rgb::RGBA8 {
    color.to_be_bytes().into()
}

pub async fn create_color(
    db: &sqlx::PgPool,
    user_id: Uuid,
    color: rgb::RGBA8,
) -> sqlx::Result<Vec<rgb::RGBA8>> {
    let color = rgba_to_i32(color);

    let colors = sqlx::query!(
        r#"
with cte as (
    insert into user_color
    (user_id, color, index)
    values ($1, $2, (select count(*) from user_color where user_id = $1)) returning color
), colors as (
    select color
    from user_color
    where user_id = $1
    order by index
)
select color as "color!" from colors
union all
select color as "color!" from cte
    "#,
        user_id,
        color
    )
    .fetch_all(db)
    .await?;

    // hack: do this in a way that maps the original stream instead of a vec (just a perf concern).
    Ok(colors
        .into_iter()
        .map(|it| color_to_rgba(it.color))
        .collect())
}

pub async fn update_color(
    db: &sqlx::PgPool,
    user_id: Uuid,
    index: u16,
    color: rgb::RGBA8,
) -> sqlx::Result<bool> {
    let color = rgba_to_i32(color);

    let mut txn = db.begin().await?;
    let exists = sqlx::query!(
        r#"
select exists(
        select 1
        from user_color
        where user_id = $1
            and index = $2
        for update
) as "exists!""#,
        user_id,
        index as i16
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        return Ok(false);
    }

    sqlx::query!(
        "update user_color set color = $3 where user_id = $1 and index = $2",
        user_id,
        index as i16,
        color
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn get_colors(db: &sqlx::PgPool, user_id: Uuid) -> sqlx::Result<Vec<rgb::RGBA8>> {
    let colors = sqlx::query!(
        r#"
select color
from user_color
where user_id = $1
order by index
"#,
        user_id
    )
    .fetch_all(db)
    .await?;

    // hack: do this in a way that maps the original stream instead of a vec (just a perf concern).
    Ok(colors
        .into_iter()
        .map(|it| color_to_rgba(it.color))
        .collect())
}

pub async fn delete_color(db: &sqlx::PgPool, user_id: Uuid, index: u16) -> sqlx::Result<()> {
    let mut txn = db.begin().await?;
    let _ = sqlx::query!(
        r#"
with delete as (
        delete from user_color
    where user_id = $1 and index = $2
)
select 1 as discard
from user_color
where user_id = $1 and index > $2
for update
"#,
        user_id,
        index as i16
    )
    .fetch_optional(&mut txn)
    .await?;

    sqlx::query!(
        r#"
update user_color
set index = index - 1
where index > $2 and user_id = $1
"#,
        user_id,
        index as i16
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn create_font(
    db: &sqlx::PgPool,
    user_id: Uuid,
    name: String,
) -> sqlx::Result<Vec<String>> {
    let font_names = sqlx::query!(
        r#"
with cte as (
    insert into user_font
    (user_id, name, index)
    values ($1, $2, (select count(*) from user_font where user_id = $1)) returning name
), names as (
    select name
    from user_font
    where user_id = $1
    order by index
)
select name as "name!" from names
union all
select name as "name!" from cte
        "#,
        user_id,
        name
    )
    .fetch_all(db)
    .await?;

    Ok(font_names.into_iter().map(|it| it.name).collect())
}

pub async fn update_font(
    db: &sqlx::PgPool,
    user_id: Uuid,
    index: u16,
    name: String,
) -> sqlx::Result<bool> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"
select exists(
        select 1
        from user_font
        where user_id = $1
            and index = $2
        for update
) as "exists!"
        "#,
        user_id,
        index as i16
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        return Ok(false);
    }

    sqlx::query!(
        r#"
update user_font
    set name = $3
    where user_id = $1
    and index = $2
        "#,
        user_id,
        index as i16,
        name
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn get_first_name(txn: &mut PgConnection, user_id: Uuid) -> sqlx::Result<String> {
    let given_name = sqlx::query!(
        r#"
select given_name
from user_profile
where user_id = $1
        "#,
        user_id
    )
    .fetch_one(txn)
    .await?
    .given_name;

    Ok(given_name)
}

pub async fn get_email(txn: &mut PgConnection, user_id: Uuid) -> sqlx::Result<String> {
    let email: Option<String> = sqlx::query!(
        r#"
select email::text
from user_email
where user_id = $1
        "#,
        user_id
    )
    .fetch_one(txn)
    .await?
    .email;

    let email = if let Some(email) = email {
        email
    } else {
        return Err(sqlx::Error::RowNotFound);
    };

    Ok(email)
}

pub async fn get_fonts(db: &sqlx::PgPool, user_id: Uuid) -> sqlx::Result<Vec<String>> {
    let font_names = sqlx::query!(
        r#"
select name
from user_font
where user_id = $1
order by index
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(font_names.into_iter().map(|it| it.name).collect())
}

pub async fn delete_font(db: &sqlx::PgPool, user_id: Uuid, index: u16) -> sqlx::Result<()> {
    let mut txn = db.begin().await?;

    let _ = sqlx::query!(
        r#"
with delete as (
        delete from user_font
    where user_id = $1 and index = $2
)
select 1 as discard
from user_font
where user_id = $1 and index > $2
for update
        "#,
        user_id,
        index as i16
    )
    .fetch_optional(&mut txn)
    .await?;

    sqlx::query!(
        r#"
update user_font
set index = index - 1
where index > $2 and user_id = $1
        "#,
        user_id,
        index as i16
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}
