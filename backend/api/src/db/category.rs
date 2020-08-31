use crate::domain::{build_tree, RawCategory};
use futures::TryStreamExt;
use shared::{
    domain::category::{Category, CategoryId},
    error::category::{CategoryDeleteError, CategoryUpdateError},
};
use sqlx::Executor;
use uuid::Uuid;

pub async fn get_top_level(db: &sqlx::PgPool) -> anyhow::Result<Vec<Category>> {
    sqlx::query!(
        r#"
select id                                                                 as "id: CategoryId",
       name,
       created_at,
       updated_at,
       (select count(*)::int8 from image_category where category_id = id) as "image_count!",
       0::int8                                                            as "jig_count!"
from category
where parent_id is null
order by index
 "#
    )
    .fetch(db)
    .map_ok(|it| Category {
        id: it.id,
        created_at: it.created_at,
        updated_at: it.updated_at,
        name: it.name,
        children: vec![],
        image_count: it.image_count as u64,
        jig_count: it.jig_count as u64,
    })
    .try_collect()
    .await
    .map_err(Into::into)
}

pub async fn get_exact(db: &sqlx::PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Category>> {
    sqlx::query!(
        r#"
select id                                                                 as "id: CategoryId",
       name,
       created_at,
       updated_at,
       (select count(*)::int8 from image_category where category_id = id) as "image_count!",
       0::int8                                                            as "jig_count!"

from category
         inner join unnest($1::uuid[]) with ordinality t(id, ord) USING (id)
order by t.ord
"#,
        ids
    )
    .fetch(db)
    .map_ok(|it| Category {
        id: it.id,
        created_at: it.created_at,
        updated_at: it.updated_at,
        name: it.name,
        children: vec![],
        image_count: it.image_count as u64,
        jig_count: it.jig_count as u64,
    })
    .try_collect()
    .await
}

pub async fn get_subtree(db: &sqlx::PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Category>> {
    sqlx::query_file_as!(RawCategory, "query/category/get_subtree.sql", ids)
        .fetch_all(db)
        .await
        .map(build_tree)
}

pub async fn get_tree(db: &sqlx::PgPool) -> sqlx::Result<Vec<Category>> {
    sqlx::query_as!(
        RawCategory,
        r#"
select id,
       parent_id,
       name,
       index,
       created_at,
       updated_at,
       (select count(*) from image_category where category_id = id)::int8 as "image_count!",
       0::int8                                                            as "jig_count!"
from category
"#
    )
    .fetch_all(db)
    .await
    .map(build_tree)
}

pub async fn get_ancestor_tree(db: &sqlx::PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Category>> {
    sqlx::query_file_as!(RawCategory, "query/category/get_ancestor_tree.sql", ids)
        .fetch_all(db)
        .await
        .map(build_tree)
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

/// checks if moving the category with `id` to have `new_parent` as it's parent would create a cycle
///
/// A cycle is where there's a category where it is it's own descendant.
async fn would_cycle(
    txn: &mut sqlx::PgConnection,
    id: Uuid,
    new_parent: Uuid,
) -> sqlx::Result<bool> {
    // trivially having `self` as your parent would cycle, as you would have a child that's yourself.
    if new_parent == id {
        return Ok(true);
    }

    sqlx::query!(
        r#"
with recursive cte(parent_id) as (
select parent_id from category where id = $1
union all
select c.parent_id from category c inner join cte on cte.parent_id = c.id
) select exists(select 1 from cte where parent_id = $2) as "would_cycle!"
    "#,
        new_parent,
        id
    )
    .fetch_one(txn)
    .await
    .map(|res| res.would_cycle)
}

pub async fn update(
    db: &sqlx::PgPool,
    CategoryId(id): CategoryId,
    parent_id: Option<Option<CategoryId>>,
    name: Option<&str>,
    index: Option<i16>,
) -> Result<(), CategoryUpdateError> {
    let current_parent = parent_id.map(|id| id.map(|it| it.0));
    let mut txn = db.begin().await?;

    txn.execute("set transaction isolation level repeatable read")
        .await?;

    let category_info = sqlx::query!(
        r#"
select parent_id, index from category where id = $1
    "#,
        id
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(CategoryUpdateError::CategoryNotFound)?;

    if let Some(name) = name {
        sqlx::query!("update category set name = $1 where id = $2", name, id)
            .execute(&mut txn)
            .await?;
    }

    let mut current_index = category_info.index;
    if let Some(parent_id) = current_parent {
        if parent_id != category_info.parent_id {
            // check that the new parent isn't a descendant (to avoid cycles)
            if let Some(new_parent) = parent_id {
                let would_cycle = would_cycle(&mut txn, id, new_parent).await?;

                if would_cycle {
                    return Err(CategoryUpdateError::Cycle);
                }
            }

            // handle the new parent not existing and return `ParentCategoryNotFound`
            let res = sqlx::query!(
                r#"
update category
set parent_id = $1,
    updated_at = now(),
    index = (select count(*)::int2 from category where parent_id is not distinct from $1)
where id = $2
returning index
"#,
                parent_id.map(|it| it),
                id
            )
            .fetch_one(&mut txn)
            .await?;

            current_index = res.index;

            backshift(&mut txn, category_info.parent_id, category_info.index, None).await?;
        }
    }

    if let Some(new_index) = index {
        let current_parent = current_parent.unwrap_or(category_info.parent_id);
        if new_index < current_index {
            sqlx::query!(
                r#"
update category
set updated_at = now(),
    index = index + 1
where index >= $1 and index < $2 and parent_id is not distinct from $3
                "#,
                new_index,
                current_index,
                current_parent
            )
            .execute(&mut txn)
            .await?;
        }

        if new_index > current_index {
            backshift(&mut txn, current_parent, current_index, Some(new_index)).await?;
        }

        if new_index != current_index {
            sqlx::query!(
                r#"
update category
set updated_at = now(),
    index = least((select count(*)::int2 from category c where c.parent_id is not distinct from parent_id), $1)
where id = $2
"#,
                index,
                id
            )
            .execute(&mut txn)
            .await?;
        }
    }

    txn.commit().await?;

    Ok(())
}

async fn backshift(
    txn: &mut sqlx::PgConnection,
    parent_id: Option<Uuid>,
    old_index: i16,
    new_index: Option<i16>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
update category
set index = index - 1, updated_at = now()
where index > $1 and index <= $2 is not false and parent_id is not distinct from $3
"#,
        old_index,
        new_index,
        parent_id
    )
    .execute(txn)
    .await
    .map(drop)
}

pub async fn delete(db: &sqlx::PgPool, id: CategoryId) -> Result<(), CategoryDeleteError> {
    let mut txn = db.begin().await?;

    txn.execute("set transaction isolation level repeatable read")
        .await?;

    let res = sqlx::query!(
        "delete from category where id = $1 returning index, parent_id",
        id.0
    )
    .fetch_optional(&mut txn)
    .await?;

    if let Some(res) = res {
        backshift(&mut txn, res.parent_id, res.index, None).await?;
    }

    txn.commit().await?;

    Ok(())
}
