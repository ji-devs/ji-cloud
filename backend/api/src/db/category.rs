use shared::category::Category;

pub async fn get(db: &sqlx::PgPool) -> sqlx::Result<Vec<Category>> {
    sqlx::query_as(
        r#"
select id, parent_id, name, "index", created_at, updated_at
from category
        "#,
    )
    .fetch_all(db)
    .await
    .map_err(|it| dbg!(it))
}
