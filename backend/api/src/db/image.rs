use chrono::{DateTime, Utc};
use shared::domain::image::{
    meta::{AffiliationId, AgeRangeId, StyleId},
    ImageId,
};
use sqlx::PgConnection;
use std::fmt::Write;
pub(crate) mod meta;

pub async fn create(
    conn: &mut PgConnection,
    source: &str,
    name: &str,
    description: &str,
    is_premium: bool,
    publish_at: Option<&DateTime<Utc>>,
) -> sqlx::Result<ImageId> {
    let id: ImageId = sqlx::query!(
        r#"
insert into image_metadata (name, source, description, is_premium, publish_at) values ($1, $2, $3, $4, $5)
returning id as "id: ImageId"
        "#,
        name,
        source,
        description,
        is_premium,
        publish_at,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

fn generate_metadata_insert(meta_kind: &str, binds: usize) -> String {
    debug_assert_ne!(binds, 0);
    debug_assert_ne!(binds, i16::MAX as usize);

    let mut s = format!(
        "insert into image_{0} (image_id, {0}_id), values($1, $2)",
        meta_kind
    );

    for i in 3..=binds {
        write!(s, ",($1,${})", i)
            .ok()
            .expect("write to String shouldn't fail");
    }

    s
}

pub async fn add_metadata(
    conn: &mut PgConnection,
    image: ImageId,
    affiliations: &[AffiliationId],
    age_ranges: &[AgeRangeId],
    styles: &[StyleId],
) -> sqlx::Result<()> {
    // todo: don't remove ones that are in the respective arrays.
    sqlx::query!("delete from image_affiliation where image_id = $1", image.0)
        .execute(&mut *conn)
        .await?;

    sqlx::query!("delete from image_age_range where image_id = $1", image.0)
        .execute(&mut *conn)
        .await?;

    sqlx::query!("delete from image_style where image_id = $1", image.0)
        .execute(&mut *conn)
        .await?;

    for affiliations in affiliations.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert("affiliation", affiliations.len());
        let mut query = sqlx::query(&query);

        for affiliation in affiliations {
            query = query.bind(image.0).bind(affiliation.0);
        }

        query.execute(&mut *conn).await?;
    }

    for age_ranges in age_ranges.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert("age_range", affiliations.len());
        let mut query = sqlx::query(&query);

        for age_range in age_ranges {
            query = query.bind(image.0).bind(age_range.0);
        }

        query.execute(&mut *conn).await?;
    }

    for styles in styles.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert("style", affiliations.len());
        let mut query = sqlx::query(&query);

        for style in styles {
            query = query.bind(image.0).bind(style.0);
        }

        query.execute(&mut *conn).await?;
    }

    Ok(())
}
