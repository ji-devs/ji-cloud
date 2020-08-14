use chrono::{DateTime, Utc};
use shared::domain::{
    category::CategoryId,
    image::{
        meta::{AffiliationId, AgeRangeId, StyleId},
        Image, ImageId,
    },
};
use sqlx::{PgConnection, PgPool};
use std::fmt::Write;
use uuid::Uuid;
pub(crate) mod meta;

trait Metadata {
    const TABLE: &'static str;
    fn as_uuid(&self) -> Uuid;
}

impl Metadata for AffiliationId {
    const TABLE: &'static str = "affiliation";

    fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Metadata for StyleId {
    const TABLE: &'static str = "style";

    fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Metadata for AgeRangeId {
    const TABLE: &'static str = "age_range";

    fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Metadata for CategoryId {
    const TABLE: &'static str = "categories";

    fn as_uuid(&self) -> Uuid {
        self.0
    }
}

pub async fn create(
    conn: &mut PgConnection,
    name: &str,
    description: &str,
    is_premium: bool,
    publish_at: Option<&DateTime<Utc>>,
) -> sqlx::Result<ImageId> {
    let id: ImageId = sqlx::query!(
        r#"
insert into image_metadata (name, description, is_premium, publish_at) values ($1, $2, $3, $4)
returning id as "id: ImageId"
        "#,
        name,
        description,
        is_premium,
        publish_at,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

async fn recycle_metadata<'a, T: Metadata>(
    conn: &mut PgConnection,
    image: ImageId,
    meta: &[T],
) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "delete from image_{} where image_id = $1",
        T::TABLE
    ))
    .bind(image.0)
    .execute(&mut *conn)
    .await?;

    for meta in meta.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert(T::TABLE, meta.len());
        let mut query = sqlx::query(&query);

        for meta in meta {
            query = query.bind(image.0).bind(meta.as_uuid());
        }

        query.execute(&mut *conn).await?;
    }

    Ok(())
}

fn generate_metadata_insert(meta_kind: &str, binds: usize) -> String {
    debug_assert_ne!(binds, 0);
    debug_assert_ne!(binds, i16::MAX as usize);

    let mut s = format!(
        "insert into image_{0} (image_id, {0}_id) values($1, $2)",
        meta_kind
    );

    for i in 3..=binds {
        write!(s, ",($1,${})", i)
            .ok()
            .expect("write to String shouldn't fail");
    }

    s
}

pub fn nul_if_empty<T>(arr: &[T]) -> Option<&[T]> {
    if arr.is_empty() {
        Some(arr)
    } else {
        None
    }
}

pub async fn update_metadata(
    conn: &mut PgConnection,
    image: ImageId,
    affiliations: Option<&[AffiliationId]>,
    age_ranges: Option<&[AgeRangeId]>,
    styles: Option<&[StyleId]>,
    categories: Option<&[CategoryId]>,
) -> sqlx::Result<()> {
    if let Some(affiliations) = affiliations {
        recycle_metadata(&mut *conn, image, affiliations).await?;
    }

    if let Some(age_ranges) = age_ranges {
        recycle_metadata(&mut *conn, image, age_ranges).await?;
    }

    if let Some(styles) = styles {
        recycle_metadata(&mut *conn, image, styles).await?;
    }

    if let Some(categories) = categories {
        recycle_metadata(&mut *conn, image, categories).await?;
    }

    Ok(())
}

pub async fn update(
    conn: &mut PgConnection,
    id: ImageId,
    name: Option<String>,
    description: Option<String>,
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
  and ($2 is distinct from name or
       $3 is distinct from description or
       $4 is distinct from is_premium)"#,
        id.0,
        name,
        description,
        is_premium
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
       description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       array((select row (category_id) from image_categories where image_id = id))     as categories,
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

pub async fn delete(db: &PgPool, image: ImageId) -> sqlx::Result<()> {
    let mut conn = db.begin().await?;

    // first, clear any metadata it might have.
    update_metadata(&mut conn, image, Some(&[]), Some(&[]), Some(&[]), Some(&[])).await?;

    // then drop.
    sqlx::query!("delete from image_metadata where id = $1", image.0)
        .execute(&mut conn)
        .await
        .map(drop)
}
