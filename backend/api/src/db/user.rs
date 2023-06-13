use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use shared::domain::{
    admin::DateFilterType,
    circle::CircleId,
    image::ImageId,
    meta::{
        AffiliationId, AgeRangeId, GoogleAddressComponent, GoogleAddressType, GoogleLocation,
        SubjectId,
    },
    user::{
        CreateProfileRequest, OtherUser, PatchProfileAdminDataRequest, PatchProfileRequest,
        UserBadge, UserId, UserProfile, UserProfileExport, UserResponse, UserScope,
    },
};
use sqlx::{PgConnection, PgPool};
use std::{convert::TryFrom, fmt::Debug, str::FromStr};
use tracing::{instrument, Instrument};
use uuid::Uuid;

pub(crate) mod public_user;

use super::{nul_if_empty, recycle_metadata};
use crate::error;

#[instrument(skip(db))]
pub async fn lookup(
    db: &sqlx::PgPool,
    id: Option<UserId>,
    name: Option<&str>,
) -> anyhow::Result<Option<OtherUser>> {
    Ok(sqlx::query_as!(
        OtherUser,
        r#"select user_id as "id: UserId" from user_profile where (user_id = $1 and $1 is not null) or (username = $2 and $2 is not null)"#,
        id.map(|x| x.0),
        name
    )
    .fetch_optional(db)
    .await?)
}

#[instrument(skip(db))]
pub async fn get_profile(db: &sqlx::PgPool, id: &UserId) -> anyhow::Result<Option<UserProfile>> {
    let row = sqlx::query!(
        //language=SQL
        r#"
select
    user_profile.user_id as "id: UserId",
    username,
    user_email.email::text as "email!",
    given_name,
    family_name,
    profile_image_id       as "profile_image?: ImageId",
    (select case when exists(select * from user_auth_google where user_id = $1) = true then 1 else 0 end)     as "is_oauth!: bool",
    languages_spoken         as "languages_spoken!: Vec<String>",
    language_app,
    language_emails,
    bio,
    badge                  as "badge!: Option<UserBadge>",
    location_public,
    languages_spoken_public,
    persona_public,
    bio_public,
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
        id.0
    )
    .fetch_optional(db)
    .await?;

    let row = match row {
        Some(row) => row,
        None => {
            return Ok(None);
        }
    };

    let user_id = row.id;

    Ok(Some(UserProfile {
        id: user_id,
        username: row.username,
        email: row.email,
        is_oauth: row.is_oauth,
        given_name: row.given_name,
        family_name: row.family_name,
        profile_image: row.profile_image,
        language_app: row.language_app,
        language_emails: row.language_emails,
        languages_spoken: row.languages_spoken,
        bio: row.bio,
        location_public: row.location_public,
        organization_public: row.organization_public,
        persona_public: row.persona_public,
        languages_spoken_public: row.languages_spoken_public,
        bio_public: row.bio_public,
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
        account_summary: None,
        badge: row.badge,
    }))
}

#[instrument(skip(db))]
pub async fn browse(
    db: &sqlx::Pool<sqlx::Postgres>,
    author_id: Option<UserId>,
    page: i32,
    page_limit: u32,
) -> sqlx::Result<Vec<UserResponse>> {
    let mut txn = db.begin().await?;

    let users = sqlx::query!(
        //language=SQL
        r#"
with cte as (
    select (array_agg("user".id))[1]
    from "user"
        left join user_profile on "user".id = user_profile.user_id
        left join user_email using(user_id)
    where ("user".id = $1 or $1 is null)
    group by "user".created_at
    order by "user".created_at desc
),
cte1 as (
    select * from unnest(array(select cte.array_agg from cte)) with ordinality t(id
   , ord) order by ord
)
select  cte1.id                 as "id!: UserId",
        username,
        given_name,
        family_name,
        user_email.email::text as "email!",
        language_emails,
        user_email.created_at  as "created_at!",
        badge                  as "badge!: Option<UserBadge>",
        organization,
        location
from cte1
        inner join user_profile on cte1.id = user_profile.user_id
        inner join user_email using(user_id)
where ord > (1 * $2 * $3)
order by ord asc
limit $3
"#,
        author_id.map(|x| x.0),
        page,
        page_limit as i32,
    )
    .fetch_all(&mut txn)
    .instrument(tracing::info_span!("query user_profile"))
    .await?;

    let v: Vec<_> = users
        .into_iter()
        .map(|user_row| {
            let location = get_location(user_row.location);

            UserResponse {
                id: user_row.id,
                username: user_row.username,
                given_name: user_row.given_name,
                family_name: user_row.family_name,
                email: user_row.email,
                city: location.city,
                state: location.state,
                country: location.country_long,
                organization: user_row.organization,
                created_at: user_row.created_at.date_naive(),
                language: user_row.language_emails,
                badge: user_row.badge,
            }
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<UserResponse>> {
    let mut txn = db.begin().await?;

    let res: Vec<_> = sqlx::query!(
        //language=SQL
        r#"
select  "user".id                 as "id!: UserId",
        username,
        given_name,
        family_name,
        user_email.email::text as "email!",
        language_emails,
        user_email.created_at  as "created_at!",
        badge                  as "badge!: Option<UserBadge>",
        organization,
        location
from "user"
inner join user_profile on "user".id = user_profile.user_id
inner join user_email on user_email.user_id = "user".id
inner join unnest($1::uuid[])
with ordinality t(id, ord) using (id)
"#,
        ids
    )
    .fetch_all(&mut txn)
    .await?;

    let v = res
        .into_iter()
        .map(|row| {
            let location = get_location(row.location);

            UserResponse {
                id: row.id,
                username: row.username,
                given_name: row.given_name,
                family_name: row.family_name,
                email: row.email,
                city: location.city,
                state: location.state,
                country: location.country_long,
                organization: row.organization,
                created_at: row.created_at.date_naive(),
                language: row.language_emails,
                badge: row.badge,
            }
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn user_profiles_by_date_range(
    db: &sqlx::PgPool,
    date_filter_type: DateFilterType,
    from_date: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
) -> anyhow::Result<Vec<UserProfileExport>> {
    let date_filter_type = serde_json::to_value(&date_filter_type)?;
    let date_filter_type: &str = date_filter_type.as_str().unwrap();

    let rows = sqlx::query!(
        //language=SQL
        r#"
select user_id              as "id!: UserId",
    username                as "username!",
    user_email.email::text  as "email!",
    given_name              as "given_name!",
    family_name             as "family_name!",
    profile_image_id        as "profile_image?: ImageId",
    language_app            as "language_app!",
    language_emails         as "language_emails!",
    languages_spoken         as "languages_spoken!: Vec<String>",
    user_profile.created_at as "created_at!",
    user_profile.updated_at,
    organization,
    persona                 as "persona!: Vec<String>",
    location,
    opt_into_edu_resources,
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
        ($3 = 'either' or $3 = 'onlynew')
        and (
            user_profile.created_at >= case when $1::timestamptz is null then to_timestamp('-infinity') else $1 end
            and user_profile.created_at < case when $2::timestamptz is null then to_timestamp('infinity') else $2 end
        )
    )
    or (
        ($3 = 'either' or $3 = 'onlyupdated')
        and (
            user_profile.updated_at >= case when $1::timestamptz is null then to_timestamp('-infinity') else $1 end
            and user_profile.updated_at < case when $2::timestamptz is null then to_timestamp('infinity') else $2 end
        )
    )
"#,
        // date_filter_type,
        from_date,
        to_date,
        date_filter_type,
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
                language_app: row.language_app,
                language_emails: row.language_emails,
                languages_spoken: row.languages_spoken,
                created_at: row.created_at,
                updated_at: row.updated_at,
                organization: row.organization,
                persona: row.persona,
                city: city.map(|c| c.into()),
                country: country.map(|c| c.into()),
                subjects: row.subjects,
                age_ranges: row.age_ranges,
                affiliations: row.affiliations,
                opt_into_edu_resources: row.opt_into_edu_resources,
            }
        })
        .collect())
}

pub async fn exists(db: &sqlx::PgPool, id: UserId) -> sqlx::Result<bool> {
    sqlx::query!(
        r#"select exists (select 1 from "user" where id = $1) as "exists!""#,
        id.0,
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
    user_id: UserId,
) -> Result<(), error::UserUpdate> {
    sqlx::query!(
        //language=SQL
        r#"
insert into user_profile
    (user_id, username, over_18, given_name, family_name, profile_image_id, language_app, language_emails, languages_spoken, timezone, opt_into_edu_resources, organization, persona, location)
values
    ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
on conflict (user_id) do update
set
    over_18 = $3,
    given_name = $4,
    family_name = $5,
    profile_image_id = $6,
    language_app = $7,
    language_emails = $8,
    languages_spoken = $9,
    timezone = $10,
    opt_into_edu_resources = $11,
    organization = $12,
    persona = $13,
    location = $14
"#,
        user_id.0,
        &req.username,
        req.over_18,
        &req.given_name,
        &req.family_name,
        profile_image_id.map(|it| it.0),
        &req.language_app,
        &req.language_emails,
        &req.languages_spoken[..],
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
        user_id.0,
        UserScope::ManageSelfAsset as i16
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
    user_id: UserId,
    req: PatchProfileRequest,
) -> Result<(), error::UserUpdate> {
    let mut txn = db.begin().await?;

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
    "#,
        user_id.0
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
            user_id.0,
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
            user_id.0,
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
            user_id.0,
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
            user_id.0,
            &persona
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update persona"))
        .await?;
    }

    if let Some(languages_spoken) = req.languages_spoken {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set languages_spoken = $2,
    updated_at = now()
where user_id = $1 and languages_spoken is distinct from $2
        "#,
            user_id.0,
            &languages_spoken
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update language spoken"))
        .await?;
    }

    sqlx::query!(
        //language=SQL
        r#"
update user_profile
set username                = coalesce($2, username),
    given_name              = coalesce($3, given_name),
    family_name             = coalesce($4, family_name),
    language_app            = coalesce($5, language_app),
    language_emails         = coalesce($6, language_emails),
    timezone                = coalesce($7, timezone),
    opt_into_edu_resources  = coalesce($8, opt_into_edu_resources),
    persona_public          = coalesce($9, persona_public),
    organization_public     = coalesce($10, organization_public),
    location_public         = coalesce($11, location_public),
    languages_spoken_public = coalesce($12, languages_spoken_public),
    bio                     = coalesce($13, bio),
    bio_public              = coalesce($14, bio_public),
    updated_at              = coalesce(now(), updated_at)
where user_id = $1
  and (($2::text is not null and $2 is distinct from username) or
       ($3::text is not null and $3 is distinct from given_name) or
       ($4::text is not null and $4 is distinct from family_name) or
       ($5::text is not null and $5 is distinct from language_app) or
       ($6::text is not null and $6 is distinct from language_emails) or
       ($7::text is not null and $7 is distinct from timezone) or
       ($8::bool is not null and $8 is distinct from opt_into_edu_resources) or
       ($9::bool is not null and $9 is distinct from persona_public) or
       ($10::bool is not null and $10 is distinct from organization_public) or
       ($11::bool is not null and $11 is distinct from location_public) or
       ($12::bool is not null and $12 is distinct from languages_spoken_public) or
       ($13::text is not null and $13 is distinct from bio) or
       ($14::bool is not null and $14 is distinct from bio_public)
    )
    "#,
        user_id.0,
        req.username,
        req.given_name,
        req.family_name,
        req.language_app,
        req.language_emails,
        req.timezone.map(|it| it.to_string()),
        req.opt_into_edu_resources,
        req.persona_public,
        req.organization_public,
        req.location_public,
        req.languages_spoken_public,
        req.bio,
        req.bio_public,
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

#[instrument(skip(db, req))]
pub async fn update_profile_admin_data(
    db: &PgPool,
    user_id: UserId,
    req: PatchProfileAdminDataRequest,
) -> Result<(), error::UserUpdate> {
    let mut txn = db.begin().await?;

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
    "#,
        user_id.0
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("user exists"))
    .await?
    .exists
    {
        return Err(error::UserUpdate::UserNotFound);
    }

    if let Some(badge) = req.badge {
        sqlx::query!(
            //language=SQL
            r#"
update user_profile
set badge      = coalesce($2, badge),
    updated_at = coalesce(now(), updated_at)
where user_id = $1
and ($2 is distinct from badge)
        "#,
            user_id.0,
            badge.map(|b| b as i16),
        )
        .execute(&mut txn)
        .instrument(tracing::info_span!("update user_profile"))
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

#[instrument(skip(conn))]
pub async fn update_metadata(
    conn: &mut PgConnection,
    user_id: UserId,
    subjects: Option<&[SubjectId]>,
    affiliations: Option<&[AffiliationId]>,
    age_ranges: Option<&[AgeRangeId]>,
) -> sqlx::Result<()> {
    const TABLE: &str = r#"user"#;

    if let Some(affiliations) = affiliations {
        recycle_metadata(&mut *conn, TABLE, user_id.0, affiliations).await?;
    }

    if let Some(age_ranges) = age_ranges {
        recycle_metadata(&mut *conn, TABLE, user_id.0, age_ranges).await?;
    }

    if let Some(subjects) = subjects {
        recycle_metadata(&mut *conn, TABLE, user_id.0, subjects).await?;
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
    user_id: UserId,
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
        user_id.0,
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
    user_id: UserId,
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
        user_id.0,
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
        user_id.0,
        index as i16,
        color
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn get_colors(db: &sqlx::PgPool, user_id: UserId) -> sqlx::Result<Vec<rgb::RGBA8>> {
    let colors = sqlx::query!(
        r#"
select color
from user_color
where user_id = $1
order by index
"#,
        user_id.0
    )
    .fetch_all(db)
    .await?;

    // hack: do this in a way that maps the original stream instead of a vec (just a perf concern).
    Ok(colors
        .into_iter()
        .map(|it| color_to_rgba(it.color))
        .collect())
}

pub async fn delete_color(db: &sqlx::PgPool, user_id: UserId, index: u16) -> sqlx::Result<()> {
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
        user_id.0,
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
        user_id.0,
        index as i16
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn create_font(
    db: &sqlx::PgPool,
    user_id: UserId,
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
        user_id.0,
        name
    )
    .fetch_all(db)
    .await?;

    Ok(font_names.into_iter().map(|it| it.name).collect())
}

pub async fn update_font(
    db: &sqlx::PgPool,
    user_id: UserId,
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
        user_id.0,
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
        user_id.0,
        index as i16,
        name
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn get_given_name(txn: &mut PgConnection, user_id: UserId) -> sqlx::Result<String> {
    let given_name = sqlx::query!(
        r#"
select given_name
from user_profile
where user_id = $1
        "#,
        user_id.0
    )
    .fetch_one(txn)
    .await?
    .given_name;

    Ok(given_name)
}

// keeping the struct here because it's so far only used here
pub struct Location {
    pub city: Option<String>,
    pub state: Option<String>,
    pub country_short: Option<String>,
    pub country_long: Option<String>,
}
pub fn get_location(location: Option<serde_json::Value>) -> Location {
    let location: Option<GoogleLocation> = location.and_then(|location_str| {
        location_str
            .as_str()
            .and_then(|location_str| serde_json::from_str(location_str).ok())
    });

    let city: Option<&GoogleAddressComponent> = location.as_ref().and_then(|l| {
        l.place
            .address_component_by_type(GoogleAddressType::Locality)
    });

    let state: Option<&GoogleAddressComponent> = location.as_ref().and_then(|l| {
        l.place
            .address_component_by_type(GoogleAddressType::AdministrativeAreaLevel1)
    });

    let country: Option<&GoogleAddressComponent> = location.as_ref().and_then(|l| {
        l.place
            .address_component_by_type(GoogleAddressType::Country)
    });

    let city = city.map(|c| c.long_name.to_string());
    let state = state.map(|c| c.short_name.to_string());
    let country_short = country.map(|c| c.short_name.to_string());
    let country_long = country.map(|c| c.long_name.to_string());

    Location {
        city,
        state,
        country_short,
        country_long,
    }
}

pub async fn get_email(txn: &mut PgConnection, user_id: UserId) -> sqlx::Result<String> {
    let email: Option<String> = sqlx::query!(
        r#"
select email::text
from user_email
where user_id = $1
        "#,
        user_id.0
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

pub async fn get_fonts(db: &sqlx::PgPool, user_id: UserId) -> sqlx::Result<Vec<String>> {
    let font_names = sqlx::query!(
        r#"
select name
from user_font
where user_id = $1
order by index
        "#,
        user_id.0
    )
    .fetch_all(db)
    .await?;

    Ok(font_names.into_iter().map(|it| it.name).collect())
}

pub async fn delete_font(db: &sqlx::PgPool, user_id: UserId, index: u16) -> sqlx::Result<()> {
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
        user_id.0,
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
        user_id.0,
        index as i16
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

// `None` here means do not filter.
#[instrument(skip(db))]
pub async fn filtered_count(db: &PgPool, user_id: Option<UserId>) -> sqlx::Result<u64> {
    let users = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select (array_agg(user_profile.user_id))[1]
            from user_profile
            left join "user" on "user".id = user_profile.user_id
            left join user_email using(user_id)
            where ("user".id = $1 or $1 is null)
            group by family_name
            order by family_name desc
        )
        select count(*) as "count!" from unnest(array(select cte.array_agg from cte)) with ordinality t(id, ord)
        "#,
        user_id.map(|it| it.0),

    )
    .fetch_one(db)
    .await?;

    Ok(users.count as u64)
}

#[instrument(skip(db))]
pub async fn has_scopes(db: &PgPool, user_id: UserId, scopes: &[UserScope]) -> sqlx::Result<bool> {
    let scopes: Vec<_> = scopes.iter().map(|scope| *scope as i16).collect();
    let authed = sqlx::query!(
        r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
        user_id.0,
        &scopes[..],
    )
    .fetch_one(db)
    .await?
    .authed;

    Ok(authed)
}
