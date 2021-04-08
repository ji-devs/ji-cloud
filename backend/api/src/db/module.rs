use std::cmp;

use shared::domain::jig::{
    module::{Module, ModuleKind},
    JigId,
};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    parent: JigId,
    kind: Option<ModuleKind>,
    body: Option<&serde_json::Value>,
) -> anyhow::Result<u16> {
    sqlx::query!(
        r#"
insert into jig_module (jig_id, kind, contents, index)
values ($1, $2, $3, (select count(*) from jig_module where jig_id = $1))
returning "index"
"#,
        parent.0,
        kind.map(|it| it as i16),
        body
    )
    .fetch_one(pool)
    .await
    .map(|it| it.index as u16)
    .map_err(Into::into)
}

pub async fn update(
    pool: &PgPool,
    parent_id: JigId,
    index: u16,
    kind: Option<ModuleKind>,
    body: Option<&serde_json::Value>,
    new_index: Option<u16>,
) -> anyhow::Result<bool> {
    let mut txn = pool.begin().await?;

    let index = index as i16;

    let exists = sqlx::query!(
        r#"select exists(select 1 from jig_module where jig_id = $1 and index = $2 for update) as "exists!""#,
        parent_id.0,
        index,
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        return Ok(false);
    }

    sqlx::query!(
        r#"
update jig_module
set contents = coalesce($3, contents),
    kind = coalesce($4, kind),
    updated_at = now()
where jig_id = $1 and index = $2 and (
    ($3::jsonb is not null and $3 is distinct from contents) or
    ($4::int2 is not null and $4 is distinct from kind)
)
"#,
        parent_id.0,
        index,
        body,
        kind.map(|it| it as i16),
    )
    .execute(&mut txn)
    .await?;

    if let Some(new_index) = new_index {
        let new_index = new_index as i16;

        // todo: don't use an extra query for this
        let max_index = sqlx::query!(
            r#"select count(*) - 1 as "max_index!" from jig_module where jig_id = $1"#,
            parent_id.0
        )
        .fetch_one(&mut txn)
        .await?
        .max_index;

        let new_index = cmp::min(new_index, max_index as i16);

        if new_index < index {
            sqlx::query!(
                r#"
update jig_module
set
    index = case when index = $2 then $3 else index + 1 end,
    updated_at = now()
where jig_id = $1 and index between $3 and $2
"#,
                parent_id.0,
                index,
                new_index
            )
            .execute(&mut txn)
            .await?;
        } else if new_index > index {
            sqlx::query!(
                r#"
update jig_module
set
    index = case when index = $2 then $3 else index - 1 end,
    updated_at = now()
where jig_id = $1 and index between $2 and $3
"#,
                parent_id.0,
                index,
                new_index
            )
            .execute(&mut txn)
            .await?;
        }
    }

    txn.commit().await?;

    Ok(true)
}

pub async fn get(pool: &PgPool, parent: JigId, index: u16) -> anyhow::Result<Option<Module>> {
    sqlx::query_as!(
        Module,
        r#"select contents as "body", kind as "kind: ModuleKind" from jig_module where jig_id = $1 and index = $2"#,
        parent.0, index as i16
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

pub async fn delete(pool: &PgPool, parent: JigId, index: u16) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let idx = sqlx::query!(
        "delete from jig_module where jig_id = $1 and index = $2 returning index",
        parent.0,
        index as i16
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.index);

    if let Some(idx) = idx {
        sqlx::query!(
            "update jig_module set index = index - 1 where jig_id = $1 and index > $2",
            parent.0,
            idx
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}
