use anyhow::Context;
use shared::domain::jig::{
    module::{Module, ModuleBody, ModuleId, ModuleKind, StableModuleId, StableOrUniqueId},
    JigId,
};
use sqlx::PgPool;
use std::cmp;

fn map_module_contents(body: &ModuleBody) -> anyhow::Result<(ModuleKind, serde_json::Value)> {
    let kind = body.kind();

    let body = match body {
        ModuleBody::CardQuiz(body) => serde_json::to_value(body)?,
        ModuleBody::Cover(body) => serde_json::to_value(body)?,
        ModuleBody::ResourceCover(body) => serde_json::to_value(body)?,
        ModuleBody::DragDrop(body) => serde_json::to_value(body)?,
        ModuleBody::Flashcards(body) => serde_json::to_value(body)?,
        ModuleBody::Matching(body) => serde_json::to_value(body)?,
        ModuleBody::MemoryGame(body) => serde_json::to_value(body)?,
        ModuleBody::Poster(body) => serde_json::to_value(body)?,
        ModuleBody::TappingBoard(body) => serde_json::to_value(body)?,
        ModuleBody::Video(body) => serde_json::to_value(body)?,
        ModuleBody::Legacy(body) => serde_json::to_value(body)?,

        _ => anyhow::bail!("Unimplemented body kind: {}", kind.as_str()),
    };

    Ok((kind, body))
}

fn transform_response_kind(
    contents: serde_json::Value,
    kind: ModuleKind,
) -> anyhow::Result<ModuleBody> {
    match kind {
        ModuleKind::CardQuiz => Ok(ModuleBody::CardQuiz(serde_json::from_value(contents)?)),
        ModuleKind::Cover => Ok(ModuleBody::Cover(serde_json::from_value(contents)?)),
        ModuleKind::ResourceCover => {
            Ok(ModuleBody::ResourceCover(serde_json::from_value(contents)?))
        }
        ModuleKind::DragDrop => Ok(ModuleBody::DragDrop(serde_json::from_value(contents)?)),
        ModuleKind::Flashcards => Ok(ModuleBody::Flashcards(serde_json::from_value(contents)?)),
        ModuleKind::Matching => Ok(ModuleBody::Matching(serde_json::from_value(contents)?)),
        ModuleKind::Memory => Ok(ModuleBody::MemoryGame(serde_json::from_value(contents)?)),
        ModuleKind::Poster => Ok(ModuleBody::Poster(serde_json::from_value(contents)?)),
        ModuleKind::TappingBoard => Ok(ModuleBody::TappingBoard(serde_json::from_value(contents)?)),
        ModuleKind::Video => Ok(ModuleBody::Video(serde_json::from_value(contents)?)),
        ModuleKind::Legacy => Ok(ModuleBody::Legacy(serde_json::from_value(contents)?)),

        _ => anyhow::bail!("Unimplemented response kind"),
    }
}

pub async fn create(
    pool: &PgPool,
    parent: JigId,
    body: ModuleBody,
) -> anyhow::Result<(ModuleId, u16)> {
    let (kind, body) = map_module_contents(&body)?;

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from jig where jig.id = $1
"#,
        parent.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let res = sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_module (jig_data_id, kind, contents, index)
values ($1, $2, $3, (select count(*) from jig_data_module where jig_data_id = $1))
returning id, "index"
"#,
        draft_id,
        kind as i16,
        body,
    )
    .fetch_one(&mut txn)
    .await
    .map(|it| (ModuleId(it.id), it.index as u16))
    .map_err(Into::into);

    txn.commit().await?;

    res
}

pub async fn get_live(
    pool: &PgPool,
    parent: JigId,
    lookup: StableOrUniqueId,
) -> anyhow::Result<Option<Module>> {
    let (unique, stable) = (lookup.unique(), lookup.stable());

    let module = sqlx::query!(
        //language=SQL
        r#"
select id          as "id: ModuleId",
       stable_id   as "stable_id: StableModuleId",
       contents    as "body",
       created_at  as "created_at",
       updated_at  as "updated_at",
       kind        as "kind: ModuleKind",
       is_complete as "is_complete"
from jig_data_module
where jig_data_module.id is not distinct from $2
   or (jig_data_id = (select live_id from jig where jig.id = $1) and stable_id is not distinct from $3)
"#,
        parent.0,
        unique.map(|it| it.0),
        stable.map(|it| it.0)
    )
    .fetch_optional(pool)
    .await?;

    let map_response = |body, kind| transform_response_kind(body, kind);

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            stable_id: it.stable_id,
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

/// FIXME dedup this from live JIG
pub async fn get_draft(
    pool: &PgPool,
    parent: JigId,
    lookup: StableOrUniqueId,
) -> anyhow::Result<Option<Module>> {
    let (unique, stable) = (lookup.unique(), lookup.stable());

    let module = sqlx::query!(
        //language=SQL
        r#"
select id          as "id: ModuleId",
       stable_id   as "stable_id: StableModuleId",
       contents    as "body",
       created_at  as "created_at",
       updated_at  as "updated_at",
       kind        as "kind: ModuleKind",
       is_complete as "is_complete"
from jig_data_module
where jig_data_module.id is not distinct from $2
   or (jig_data_id = (select draft_id from jig where jig.id = $1) and stable_id is not distinct from $3)
"#,
        parent.0,
        unique.map(|it| it.0),
        stable.map(|it| it.0)
    )
        .fetch_optional(pool)
        .await?;

    let map_response = |body, kind| transform_response_kind(body, kind);

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            stable_id: it.stable_id,
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
    parent_id: JigId,
    lookup: StableOrUniqueId,
    body: Option<&ModuleBody>,
    new_index: Option<u16>,
    is_complete: Option<bool>,
) -> anyhow::Result<bool> {
    let (unique, stable) = (lookup.unique(), lookup.stable());

    let (kind, body) = match body.map(map_module_contents).transpose()? {
        Some((kind, body)) => (Some(kind), Some(body)),
        None => (None, None),
    };

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from jig where jig.id = $1
"#,
        parent_id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.draft_id);

    let index = sqlx::query!(
        //language=SQL
        r#"
select index from jig_data_module
where jig_data_module.id is not distinct from $2
   or (jig_data_id = $1 and stable_id is not distinct from $3)
"#,
        draft_id,
        unique.map(|it| it.0),
        stable.map(|it| it.0),
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
update jig_data_module
set contents    = coalesce($3, contents),
    kind        = coalesce($4, kind),
    is_complete = coalesce($5, is_complete)
where jig_data_id = $1
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
            r#"select count(*) - 1 as "max_index!" from jig_data_module where jig_data_id = $1"#,
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
update jig_data_module
set
    index = case when index = $2 then $3 else index + 1 end,
    updated_at = now()
where jig_data_id = $1 and index between $3 and $2
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
update jig_data_module
set
    index = case when index = $2 then $3 else index - 1 end,
    updated_at = now()
where jig_data_id = $1 and index between $2 and $3
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

pub async fn delete(pool: &PgPool, parent: JigId, lookup: StableOrUniqueId) -> anyhow::Result<()> {
    let (unique, stable) = (lookup.unique(), lookup.stable());

    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from jig where jig.id = $1
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
from jig_data_module
where jig_data_module.id is not distinct from $2
   or (jig_data_id = $1 and stable_id is not distinct from $3)
returning index
"#,
        draft_id,
        unique.map(|it| it.0),
        stable.map(|it| it.0),
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.index);

    if let Some(idx) = idx {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data_module
set index = index - 1
where jig_data_id = $1
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
