use shared::category::{Category, CategoryDeleteError, CategoryId};
use sqlx::{postgres::PgDatabaseError, Executor};

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

pub async fn delete(db: &sqlx::PgPool, id: CategoryId) -> Result<(), CategoryDeleteError> {
    let mut txn = db
        .begin()
        .await
        .map_err(|_| CategoryDeleteError::InternalServerError)?;

    txn.execute("set transaction isolation level repeatable read")
        .await
        .map_err(|_| CategoryDeleteError::InternalServerError)?;

    let res = sqlx::query!(
        r#"
delete from category where id = $1
returning index, parent_id
    "#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await
    .map_err(|err| {
        return match err {
            sqlx::Error::Database(err)
                if err.downcast_ref::<PgDatabaseError>().constraint()
                    == Some("category_parent_id_fkey") =>
            {
                CategoryDeleteError::Children
            }
            _ => CategoryDeleteError::InternalServerError,
        };
    })?
    .ok_or(CategoryDeleteError::CategoryNotFound)?;

    sqlx::query!(
        r#"
update category
set index = index - 1, updated_at = now()
where index > $1 and parent_id is not distinct from $2
"#,
        res.index,
        res.parent_id
    )
    .execute(&mut txn)
    .await
    .map_err(|_| CategoryDeleteError::InternalServerError)?;

    txn.commit()
        .await
        .map_err(|_| CategoryDeleteError::InternalServerError)?;

    Ok(())
}
