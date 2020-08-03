use shared::category::{Category, CategoryId};

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

pub async fn create(
    db: &sqlx::PgPool,
    name: &str,
    parent_id: Option<CategoryId>,
) -> sqlx::Result<(CategoryId, u16)> {
    let res = sqlx::query!(
        r#"
insert into category (index, parent_id, name)
VALUES((select count(*)::int2 from category where parent_id is not distinct from $1), $1, $2)
returning index, id"#,
        parent_id.map(|it| it.0),
        name,
    )
    .fetch_one(db)
    .await?;

    Ok((CategoryId(res.id), res.index as u16))
}
