use super::recycle_metadata;
use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use shared::domain::{
    category::CategoryId,
    image::{Image, ImageId, ImageKind},
    meta::{AffiliationId, AgeRangeId, StyleId},
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub mod user {
    use futures::stream::BoxStream;
    use shared::domain::image::{user::UserImage, ImageId};
    use sqlx::PgPool;

    pub async fn create(conn: &PgPool) -> sqlx::Result<ImageId> {
        let id: ImageId = sqlx::query!(
            r#"
insert into user_image_library default values
returning id as "id: ImageId"
"#,
        )
        .fetch_one(conn)
        .await?
        .id;

        Ok(id)
    }

    pub async fn delete(db: &PgPool, image: ImageId) -> sqlx::Result<()> {
        sqlx::query!("delete from user_image_library where id = $1", image.0)
            .execute(db)
            .await
            .map(drop)
    }

    pub async fn get(db: &PgPool, image: ImageId) -> sqlx::Result<Option<UserImage>> {
        sqlx::query_as!(
            UserImage,
            r#"select id as "id: ImageId" from user_image_library where id = $1"#,
            image.0
        )
        .fetch_optional(db)
        .await
    }

    pub fn list(db: &PgPool) -> BoxStream<'_, sqlx::Result<UserImage>> {
        sqlx::query_as!(
            UserImage,
            r#"select id as "id: ImageId" from user_image_library order by created_at desc"#,
        )
        .fetch(db)
    }
}

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
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

pub async fn update_metadata(
    conn: &mut PgConnection,
    image: ImageId,
    affiliations: Option<&[AffiliationId]>,
    age_ranges: Option<&[AgeRangeId]>,
    styles: Option<&[StyleId]>,
    categories: Option<&[CategoryId]>,
) -> sqlx::Result<()> {
    const TABLE: &str = "image";

    if let Some(affiliations) = affiliations {
        recycle_metadata(&mut *conn, TABLE, image.0, affiliations).await?;
    }

    if let Some(age_ranges) = age_ranges {
        recycle_metadata(&mut *conn, TABLE, image.0, age_ranges).await?;
    }

    if let Some(styles) = styles {
        recycle_metadata(&mut *conn, TABLE, image.0, styles).await?;
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
) -> sqlx::Result<bool> {
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

    sqlx::query!(
        r#"
update image_metadata
set name        = coalesce($2, name),
    description = coalesce($3, description),
    is_premium  = coalesce($4, is_premium),
    updated_at  = now()
where id = $1
  and (($2::text is not null and $2 is distinct from name) or
       ($3::text is not null and $3 is distinct from description) or
       ($4::boolean is not null and $4 is distinct from is_premium))"#,
        id.0,
        name,
        description,
        is_premium,
    )
    .execute(conn)
    .await?;

    Ok(true)
}

pub async fn get_one(db: &PgPool, id: ImageId) -> sqlx::Result<Option<Image>> {
    sqlx::query_as(
r#"
select id,
       name,
       kind,
       description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations
from image_metadata
where id = $1
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
) -> BoxStream<'_, sqlx::Result<Image>> {
    sqlx::query_as(
r#"
select id,
       kind,
       name,
       description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations
from image_metadata
where 
    publish_at < now() is not distinct from $1 or $1 is null
    and kind = $3 is not distinct from $3 or $3 is null
order by coalesce(updated_at, created_at) desc
limit 20 offset 20 * $2
"#)
    .bind(is_published)
    .bind(page)
    .bind(kind.map(|it| it as i16))
    .fetch(db)
}

pub async fn filtered_count(db: &PgPool, is_published: Option<bool>) -> sqlx::Result<u64> {
    sqlx::query!(r#"select count(*) as "count!: i64" from image_metadata where publish_at < now() is not distinct from $1 or $1 is null"#, is_published)
        .fetch_one(db)
        .await
        .map(|it| it.count as u64)
}

pub fn get<'a>(db: &'a PgPool, ids: &'a [Uuid]) -> BoxStream<'a, sqlx::Result<Image>> {
    sqlx::query_as(
r#"
select id,
       kind,
       name,
       description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_category where image_id = id))       as categories,
       array((select row (style_id) from image_style where image_id = id))             as styles,
       array((select row (age_range_id) from image_age_range where image_id = id))     as age_ranges,
       array((select row (affiliation_id) from image_affiliation where image_id = id)) as affiliations
from image_metadata
inner join unnest($1::uuid[]) with ordinality t(id, ord) USING (id)
order by t.ord
"#).bind(ids)
    .fetch(db)
}

pub async fn delete(db: &PgPool, image: ImageId) -> sqlx::Result<()> {
    let mut conn = db.begin().await?;

    // first, clear any metadata it might have.
    update_metadata(&mut conn, image, Some(&[]), Some(&[]), Some(&[]), Some(&[])).await?;

    // then drop.
    sqlx::query!("delete from image_metadata where id = $1", image.0)
        .execute(&mut conn)
        .await?;
    conn.commit().await
}
