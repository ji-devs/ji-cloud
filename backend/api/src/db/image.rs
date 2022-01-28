use super::{recycle_metadata, recycle_tags};
use crate::translate::multi_translation;
use anyhow::Context;
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use serde_json::json;
use shared::domain::{
    category::CategoryId,
    image::{ImageId, ImageKind, ImageMetadata},
    meta::{AffiliationId, AgeRangeId, ImageStyleId, ImageTagIndex},
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub mod recent;
pub mod tag;
pub mod user;

pub async fn create(
    conn: &mut PgConnection,
    name: &str,
    description: &str,
    is_premium: bool,
    publish_at: Option<DateTime<Utc>>,
    kind: ImageKind,
) -> sqlx::Result<ImageId> {
    let id: ImageId = sqlx::query!(
        r#"
insert into image_metadata (name, description, is_premium, publish_at, kind) values ($1, $2, $3, $4, $5)
returning id as "id: ImageId"
        "#,
        name,
        description,
        is_premium,
        publish_at,
        kind as i16,
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    sqlx::query!("insert into image_upload (image_id) values($1)", id.0)
        .execute(&mut *conn)
        .await?;

    Ok(id)
}

pub async fn update_metadata(
    conn: &mut PgConnection,
    image: ImageId,
    affiliations: Option<&[AffiliationId]>,
    age_ranges: Option<&[AgeRangeId]>,
    image_styles: Option<&[ImageStyleId]>,
    categories: Option<&[CategoryId]>,
    image_tags: Option<&[ImageTagIndex]>,
) -> sqlx::Result<()> {
    const TABLE: &str = "image";

    if let Some(affiliations) = affiliations {
        recycle_metadata(&mut *conn, TABLE, image.0, affiliations).await?;
    }

    if let Some(age_ranges) = age_ranges {
        recycle_metadata(&mut *conn, TABLE, image.0, age_ranges).await?;
    }

    if let Some(styles) = image_styles {
        recycle_metadata(&mut *conn, TABLE, image.0, styles).await?;
    }

    if let Some(tags) = image_tags {
        recycle_tags(&mut *conn, TABLE, image.0, tags).await?;
    }

    if let Some(categories) = categories {
        recycle_metadata(&mut *conn, TABLE, image.0, categories).await?;
    }

    Ok(())
}

pub async fn update(
    conn: &mut PgConnection,
    id: ImageId,
    name: Option<&str>,
    description: Option<&str>,
    is_premium: Option<bool>,
    publish_at: Option<Option<DateTime<Utc>>>,
    api_key: &Option<String>,
) -> anyhow::Result<bool> {
    if !sqlx::query!(
        r#"select exists(select 1 from image_metadata where id = $1) as "exists!""#,
        id.0
    )
    .fetch_one(&mut *conn)
    .await?
    .exists
    {
        return Ok(false);
    }

    if let Some(publish_at) = publish_at {
        sqlx::query!(
            r#"
update image_metadata
set publish_at = $2, updated_at = now()
where id = $1 and $2 is distinct from publish_at"#,
            id.0,
            publish_at
        )
        .execute(&mut *conn)
        .await?;
    }

    if let Some(description) = description {
        let translate_text = match &api_key {
            Some(key) => multi_translation(description, key)
                .await
                .context("could not translate text")?,
            None => None,
        };

        sqlx::query!(
            r#"
update image_metadata
set description = $2,
    translated_description = (case when ($3::jsonb is not null) then $3::jsonb else (translated_description) end),
    updated_at = now()
where id = $1 and $2 is distinct from description"#,
            id.0,
            description,
            json!(translate_text)
        )
        .execute(&mut *conn)
        .await?;
    }

    sqlx::query!(
        //language=SQL
        r#"
update image_metadata
set name        = coalesce($2, name),
    is_premium  = coalesce($3, is_premium),
    updated_at  = now()
where id = $1
  and (($2::text is not null and $2 is distinct from name) or
       ($3::boolean is not null and $3 is distinct from is_premium))"#,
        id.0,
        name,
        is_premium,
    )
    .execute(conn)
    .await?;

    Ok(true)
}

pub async fn get_one(db: &PgPool, id: ImageId) -> sqlx::Result<Option<ImageMetadata>> {
    sqlx::query_as(
r#"
select id,
       name,
       kind,
       description,
       translated_description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       translated_description,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations,
       array((select row (tag_index) from image_tag_join where image_id = id))         as tags
from image_metadata
         join image_upload on id = image_id
where id = $1
  and processing_result is true
"#)
    .bind(id)
    .fetch_optional(db)
    .await
}

pub fn list(
    db: &PgPool,
    is_published: Option<bool>,
    kind: Option<ImageKind>,
    page: i32,
) -> BoxStream<'_, sqlx::Result<ImageMetadata>> {
    sqlx::query_as(
r#"
select id,
       kind,
       name,
       description,
       translated_description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations,
       array((select row (tag_index) from image_tag_join where image_id = id))         as tags
from (image_metadata
         inner join image_upload on image_id = id)
where processing_result is not distinct from true 
    and (publish_at < now() is not distinct from $1 or $1 is null)
    and (kind is not distinct from $3 or $3 is null) 
order by coalesce(updated_at, created_at) desc
limit 20 offset 20 * $2
"#)
    .bind(is_published)
    .bind(page)
    .bind(kind.map(|it| it as i16))
    .fetch(db)
}

pub async fn filtered_count(
    db: &PgPool,
    is_published: Option<bool>,
    kind: Option<ImageKind>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        r#"
select count(*) as "count!: i64" 
from image_metadata
        inner join image_upload on image_id = id 
where processing_result is not distinct from true 
    and (publish_at < now() is not distinct from $1 or $1 is null)
    and (kind is not distinct from $2 or $2 is null)"#,
        is_published,
        kind.map(|it| it as i16),
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub fn get<'a>(db: &'a PgPool, ids: &'a [Uuid]) -> BoxStream<'a, sqlx::Result<ImageMetadata>> {
    sqlx::query_as(
r#"
select id,
       kind,
       name,
       description,
       translated_description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations,
       array((select row (tag_index) from image_tag_join where image_id = id))         as tags
from image_metadata
         inner join image_upload on image_id = id
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) USING (id)
where processing_result is not distinct from true
order by random()
"#).bind(ids)
    .fetch(db)
}

pub async fn delete(db: &PgPool, image: ImageId) -> sqlx::Result<()> {
    let mut conn = db.begin().await?;

    // first, clear any metadata it might have.
    update_metadata(
        &mut conn,
        image,
        Some(&[]),
        Some(&[]),
        Some(&[]),
        Some(&[]),
        Some(&[]),
    )
    .await?;

    sqlx::query!("delete from image_upload where image_id = $1", image.0)
        .execute(&mut conn)
        .await?;

    // then drop.
    sqlx::query!("delete from image_metadata where id = $1", image.0)
        .execute(&mut conn)
        .await?;

    conn.commit().await
}
