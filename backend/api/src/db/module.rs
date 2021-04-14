use std::cmp;

use shared::domain::jig::{
    module::{Module, ModuleBody, ModuleBodyResponse, ModuleId, ModuleIdOrIndex, ModuleKind},
    JigId,
};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    parent: JigId,
    body: Option<&ModuleBody>,
) -> anyhow::Result<(ModuleId, u16)> {
    // note: should convert `unknowns` to <insert known here> if possible.
    let (kind, body) = match body {
        Some(it) => {
            log::warn!(
                "Converting known body into unknown body: {}",
                it.kind().as_str()
            );

            (Some(it.kind()), Some(it.body_to_json()?))
        }
        None => (None, None),
    };

    sqlx::query!(
        r#"
insert into jig_module (jig_id, kind, contents, index)
values ($1, $2, $3, (select count(*) from jig_module where jig_id = $1))
returning id, "index"
"#,
        parent.0,
        kind.map(|it| it as i16),
        body.as_deref(),
    )
    .fetch_one(pool)
    .await
    .map(|it| (ModuleId(it.id), it.index as u16))
    .map_err(Into::into)
}

pub async fn update(
    pool: &PgPool,
    parent_id: JigId,
    lookup: ModuleIdOrIndex,
    body: Option<&ModuleBody>,
    new_index: Option<u16>,
) -> anyhow::Result<bool> {
    let (id, index) = (lookup.id(), lookup.index());

    // todo: merge with above.
    // note: should convert `unknowns` to <insert known here> if possible.
    let (kind, body) = match dbg!(body) {
        Some(it) => {
            log::warn!(
                "Converting known body into unknown body: {}",
                it.kind().as_str()
            );

            (Some(it.kind()), Some(it.body_to_json()?))
        }
        None => (None, None),
    };

    let mut txn = pool.begin().await?;

    let index = sqlx::query!(
        r#"select index from jig_module where jig_id = $1 and (id is not distinct from $2 or index is not distinct from $3)"#,
        parent_id.0,
        id.map(|it| it.0),
        index.map(|it| it as i16)
    )
    .fetch_optional(&mut txn)
    .await?;

    let index = match index {
        Some(it) => it.index,
        None => return Ok(false),
    };

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
        body.as_deref(),
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

fn transform_response_kind(
    contents: Option<serde_json::Value>,
    kind: Option<ModuleKind>,
) -> Option<ModuleBodyResponse> {
    match (kind, contents) {
        (None, _) => None,
        (Some(ModuleKind::Cover), body) => Some(ModuleBodyResponse::Cover(body)),
        (Some(_), None) => None,
        (Some(kind), Some(body)) => Some(ModuleBodyResponse::Unknown { kind, body }),
    }
}

pub async fn get(
    pool: &PgPool,
    parent: JigId,
    lookup: ModuleIdOrIndex,
) -> sqlx::Result<Option<Module>> {
    let (id, index) = (lookup.id(), lookup.index());

    let module = sqlx::query!(
        r#"
select 
    id as "id: ModuleId",
    contents as "body",
    kind as "kind: ModuleKind"
from jig_module
where jig_id = $1 and (id is not distinct from $2 or index is not distinct from $3)
"#,
        parent.0,
        id.map(|it| it.0),
        index.map(|it| it as i16)
    )
    .fetch_optional(pool)
    .await?;

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            body: transform_response_kind(it.body, it.kind),
        })),
        None => Ok(None),
    }
}

pub async fn delete(pool: &PgPool, parent: JigId, lookup: ModuleIdOrIndex) -> anyhow::Result<()> {
    let (id, index) = (lookup.id(), lookup.index());
    let mut txn = pool.begin().await?;

    let idx = sqlx::query!(
        "delete from jig_module where jig_id = $1 and (id is not distinct from $2 or index is not distinct from $3) returning index",
        parent.0,
        id.map(|it| it.0),
        index.map(|it| it as i16),
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
