use shared::domain::image::meta::{
    Affiliation, AffiliationId, AgeRange, AgeRangeId, Style, StyleId,
};
use sqlx::PgPool;

pub async fn get_style(db: &PgPool) -> sqlx::Result<Vec<Style>> {
    sqlx::query_as!(
        Style,
        r#"select id as "id: StyleId", created_at, updated_at from style"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_age_ranges(db: &PgPool) -> sqlx::Result<Vec<AgeRange>> {
    sqlx::query_as!(
        AgeRange,
        r#"select id as "id: AgeRangeId", created_at, updated_at from style"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_affiliations(db: &PgPool) -> sqlx::Result<Vec<Affiliation>> {
    sqlx::query_as!(
        Affiliation,
        r#"select id as "id: AffiliationId", created_at, updated_at from style"#
    )
    .fetch_all(db)
    .await
}
