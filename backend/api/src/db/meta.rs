use shared::domain::meta::{
    Affiliation, AffiliationId, AgeRange, AgeRangeId, Style, StyleId, Subject, SubjectId,
};
use sqlx::PgPool;

pub async fn get_style(db: &PgPool) -> sqlx::Result<Vec<Style>> {
    sqlx::query_as!(
        Style,
        r#"select id as "id: StyleId", display_name, created_at, updated_at from style"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_age_ranges(db: &PgPool) -> sqlx::Result<Vec<AgeRange>> {
    sqlx::query_as!(
        AgeRange,
        r#"select id as "id: AgeRangeId", display_name, created_at, updated_at from age_range"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_affiliations(db: &PgPool) -> sqlx::Result<Vec<Affiliation>> {
    sqlx::query_as!(
        Affiliation,
        r#"select id as "id: AffiliationId", display_name, created_at, updated_at from affiliation"#
    )
    .fetch_all(db)
    .await
}

pub async fn get_subjects(db: &PgPool) -> sqlx::Result<Vec<Subject>> {
    sqlx::query_as!(
        Subject,
        r#"select subject_id as "id: SubjectId", display_name, created_at, updated_at from subject"#
    )
    .fetch_all(db)
    .await
}
