use anyhow::Context;
use shared::domain::jig::{
    module::{Module, ModuleBody, ModuleId, ModuleIdOrIndex, ModuleKind},
    JigId,
};
use sqlx::PgPool;
use std::cmp;

pub async fn create(
    pool: &PgPool,
    parent: JigId,
    body: ModuleBody,
) -> anyhow::Result<(ModuleId, u16)> {
    let kind = body.kind();
    let body = serde_json::to_value(body)?;

    sqlx::query!(
        r#"
insert into jig_module (jig_id, kind, contents, index)
values ($1, $2, $3, (select count(*) from jig_module where jig_id = $1))
returning id, "index"
"#,
        parent.0,
        kind as i16,
        body,
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
    is_complete: Option<bool>,
) -> anyhow::Result<bool> {
    let (id, index) = (lookup.id(), lookup.index());

    let (kind, body) = match body.map(map_module_contents).transpose()? {
        Some((kind, body)) => (Some(kind), Some(body)),
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
    is_complete = coalesce($5, is_complete)
where jig_id = $1 and index = $2
"#,
        parent_id.0,
        index,
        body.as_ref(),
        kind.map(|it| it as i16),
        is_complete,
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

fn map_module_contents(body: &ModuleBody) -> anyhow::Result<(ModuleKind, serde_json::Value)> {
    let kind = body.kind();

    let body = match body {
        ModuleBody::CardQuiz(body) => serde_json::to_value(body)?,
        ModuleBody::Cover(body) => serde_json::to_value(body)?,
        ModuleBody::Flashcards(body) => serde_json::to_value(body)?,
        ModuleBody::Matching(body) => serde_json::to_value(body)?,
        ModuleBody::MemoryGame(body) => serde_json::to_value(body)?,
        ModuleBody::Poster(body) => serde_json::to_value(body)?,
        ModuleBody::TappingBoard(body) => serde_json::to_value(body)?,
        ModuleBody::DragDrop(body) => serde_json::to_value(body)?,

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
        ModuleKind::Flashcards => Ok(ModuleBody::Flashcards(serde_json::from_value(contents)?)),
        ModuleKind::Matching => Ok(ModuleBody::Matching(serde_json::from_value(contents)?)),
        ModuleKind::Memory => Ok(ModuleBody::MemoryGame(serde_json::from_value(contents)?)),
        ModuleKind::Poster => Ok(ModuleBody::Poster(serde_json::from_value(contents)?)),
        ModuleKind::TappingBoard => Ok(ModuleBody::TappingBoard(serde_json::from_value(contents)?)),
        ModuleKind::DragDrop => Ok(ModuleBody::DragDrop(serde_json::from_value(contents)?)),

        _ => anyhow::bail!("Unimplemented response kind"),
    }
}

pub async fn get(
    pool: &PgPool,
    parent: JigId,
    lookup: ModuleIdOrIndex,
) -> anyhow::Result<Option<Module>> {
    let (id, index) = (lookup.id(), lookup.index());

    let module = sqlx::query!(
        r#"
select 
    id as "id: ModuleId",
    contents as "body",
    kind as "kind: ModuleKind",
    is_complete as "is_complete"
from jig_module
where jig_id = $1 and (id is not distinct from $2 or index is not distinct from $3)
"#,
        parent.0,
        id.map(|it| it.0),
        index.map(|it| it as i16)
    )
    .fetch_optional(pool)
    .await?;

    let map_response = |body, kind| transform_response_kind(body, kind);

    match module {
        Some(it) => Ok(Some(Module {
            id: it.id,
            body: map_response(it.body, it.kind).context(anyhow::anyhow!(
                "failed to transform module of kind {:?}",
                it.kind
            ))?,
            is_complete: it.is_complete,
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
