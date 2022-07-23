use anyhow::Context;
use shared::domain::{
    module::{Module, ModuleBody, ModuleId, ModuleKind},
    resource::ResourceId,
};
use sqlx::PgPool;
use std::cmp;

pub async fn create(
    pool: &PgPool,
    parent: ResourceId,
    body: ModuleBody,
    is_complete: bool,
) -> anyhow::Result<(ModuleId, u16)> {
    let (kind, body) = ModuleBody::map_module_contents(&body)?;

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from resource where resource.id = $1
"#,
        parent.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let res = sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_module (resource_data_id, kind, contents, index, is_complete)
values ($1, $2, $3, (select count(*) from resource_data_module where resource_data_id = $1), $4)
returning id, "index"
"#,
        draft_id,
        kind as i16,
        body,
        is_complete,
    )
    .fetch_one(&mut txn)
    .await
    .map(|it| (ModuleId(it.id), it.index as u16))
    .map_err(Into::into);

    txn.commit().await?;

    res
}

pub async fn get_live(pool: &PgPool, id: ModuleId) -> anyhow::Result<Option<Module>> {
    let module = sqlx::query!(
        //language=SQL
        r#"
select jdm.id          as "id!: ModuleId",
       contents    as "body!",
       created_at  as "created_at!",
       updated_at  as "updated_at!",
       kind        as "kind!: ModuleKind",
       is_complete as "is_complete!"
from resource_data_module "jdm"
inner join resource on resource.live_id = jdm.resource_data_id 
where jdm.id is not distinct from $1 
"#,
        id.0,
    )
    .fetch_optional(pool)
    .await?;

    let map_response = |body, kind| ModuleBody::transform_response_kind(body, kind);

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            created_at: it.created_at,
            updated_at: it.updated_at,
            body: map_response(it.body, it.kind).context(anyhow::anyhow!(
                "failed to transform module of kind {:?}",
                it.kind
            ))?,
            is_complete: it.is_complete,
            is_updated: it.created_at < it.updated_at,
        })),
        None => Ok(None),
    }
}

pub async fn get_draft(pool: &PgPool, id: ModuleId) -> anyhow::Result<Option<Module>> {
    let module = sqlx::query!(
        //language=SQL
        r#"
select jdm.id          as "id!: ModuleId",
       contents    as "body!",
       created_at  as "created_at!",
       updated_at  as "updated_at!",
       kind        as "kind!: ModuleKind",
       is_complete as "is_complete!"
from resource_data_module "jdm"
inner join resource on resource.draft_id = jdm.resource_data_id 
where jdm.id is not distinct from $1 
"#,
        id.0,
    )
    .fetch_optional(pool)
    .await?;

    let map_response = |body, kind| ModuleBody::transform_response_kind(body, kind);

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            created_at: it.created_at,
            updated_at: it.updated_at,
            body: map_response(it.body, it.kind).context(anyhow::anyhow!(
                "failed to transform module of kind {:?}",
                it.kind
            ))?,
            is_complete: it.is_complete,
            is_updated: it.created_at < it.updated_at,
        })),
        None => Ok(None),
    }
}

pub async fn update(
    pool: &PgPool,
    parent_id: ResourceId,
    id: ModuleId,
    body: Option<&ModuleBody>,
    new_index: Option<u16>,
    is_complete: Option<bool>,
) -> anyhow::Result<bool> {
    let (kind, body) = match body.map(ModuleBody::map_module_contents).transpose()? {
        Some((kind, body)) => (Some(kind), Some(body)),
        None => (None, None),
    };

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from resource where resource.id = $1
"#,
        parent_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let index = sqlx::query!(
        //language=SQL
        r#"
select index from resource_data_module
where resource_data_id = $1 and resource_data_module.id is not distinct from $2
"#,
        draft_id,
        id.0,
    )
    .fetch_optional(&mut txn)
    .await?;

    let index = match index {
        Some(it) => it.index,
        None => return Ok(false),
    };

    sqlx::query!(
        //language=SQL
        r#"
update resource_data_module
set contents    = coalesce($3, contents),
    kind        = coalesce($4, kind),
    is_complete = coalesce($5, is_complete)
where resource_data_id = $1
  and index = $2
"#,
        draft_id,
        index,
        body.as_ref(),
        kind.map(|it| it as i16),
        is_complete,
    )
    .execute(&mut txn)
    .await?;

    if let Some(new_index) = new_index {
        let new_index = new_index as i16;

        let max_index = sqlx::query!(
            //language=SQL
            r#"select count(*) - 1 as "max_index!" from resource_data_module where resource_data_id = $1"#,
            draft_id
        )
        .fetch_one(&mut txn)
        .await?
        .max_index;

        let new_index = cmp::min(new_index, max_index as i16);

        if new_index < index {
            sqlx::query!(
                //language=SQL
                r#"
update resource_data_module
set
    index = case when index = $2 then $3 else index + 1 end,
    updated_at = now()
where resource_data_id = $1 and index between $3 and $2
"#,
                draft_id,
                index,
                new_index
            )
            .execute(&mut txn)
            .await?;
        } else if new_index > index {
            sqlx::query!(
                //language=SQL
                r#"
update resource_data_module
set
    index = case when index = $2 then $3 else index - 1 end,
    updated_at = now()
where resource_data_id = $1 and index between $2 and $3
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

pub async fn delete(pool: &PgPool, parent: ResourceId, id: ModuleId) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from resource where resource.id = $1
"#,
        parent.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let idx = sqlx::query!(
        //language=SQL
        r#"
delete
from resource_data_module
where resource_data_id = $1 and resource_data_module.id is not distinct from $2
returning index
"#,
        draft_id,
        id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.index);

    if let Some(idx) = idx {
        sqlx::query!(
            //language=SQL
            r#"
update resource_data_module
set index = index - 1
where resource_data_id = $1
  and index > $2
"#,
            draft_id,
            idx,
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}
