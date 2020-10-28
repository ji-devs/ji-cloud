use std::borrow::Cow;

use chrono::{DateTime, Utc};
use shared::domain::jig::{Jig, JigId, LiteModule, ModuleId, ModuleKind};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

// todo: move this to a `module` mod.
async fn module_of_kind(
    conn: &mut PgConnection,
    kind: Option<ModuleKind>,
) -> anyhow::Result<ModuleId> {
    sqlx::query!(
        r#"
insert into module (kind)
values ($1)
returning id as "id: ModuleId"
"#,
        kind.map(|it| it as i16)
    )
    .fetch_one(&mut *conn)
    .await
    .map(|it| it.id)
    .map_err(Into::into)
}

pub async fn create(
    pool: &PgPool,
    display_name: Option<&str>,
    cover_id: Option<ModuleId>,
    module_ids: &[ModuleId],
    ending_id: Option<ModuleId>,
    creator_id: Uuid,
    publish_at: Option<DateTime<Utc>>,
) -> anyhow::Result<JigId> {
    let mut transaction = pool.begin().await?;

    let cover_id = match cover_id {
        Some(id) => id,
        None => module_of_kind(&mut transaction, Some(ModuleKind::DesignPage)).await?,
    };

    let ending_id = match ending_id {
        Some(id) => id,
        None => module_of_kind(&mut transaction, Some(ModuleKind::DesignPage)).await?,
    };

    let module_ids = match module_ids.is_empty() {
        false => Cow::Borrowed(module_ids),
        true => Cow::Owned(vec![module_of_kind(&mut transaction, None).await?]),
    };

    let jig = sqlx::query!(
        r#"
insert into jig
    (display_name, cover_id, ending_id, creator_id, author_id, publish_at)
values ($1, $2, $3, $4, $4, $5)
returning id
"#,
        display_name,
        cover_id.0,
        ending_id.0,
        creator_id,
        publish_at
    )
    .fetch_one(&mut transaction)
    .await?;

    // todo: batch
    for (idx, module_id) in module_ids.iter().enumerate() {
        sqlx::query!(
            r#"
insert into jig_module (jig_id, "index", module_id)
values ($1, $2, $3)"#,
            jig.id,
            idx as i16,
            module_id.0
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(JigId(jig.id))
}

pub async fn get(pool: &PgPool, id: JigId) -> anyhow::Result<Option<Jig>> {
    let jig = sqlx::query!(
        r#"
select id                                             as "id: JigId",
       display_name,
       cover_id                                       as "cover_id: ModuleId",
       (select kind from module where id = cover_id)  as "cover_kind: ModuleKind",
       ending_id                                      as "ending_id: ModuleId",
       (select kind from module where id = ending_id) as "ending_kind: ModuleKind",
       creator_id,
       author_id,
       publish_at,
       array(select row (module_id, kind)
             from jig_module
                      inner join module on module_id = module.id
             where jig_id = $1
             order by "index")                        as "modules!: Vec<(ModuleId, Option<ModuleKind>)>"
from jig
where id = $1"#,
        id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| Jig {
        id: row.id,
        display_name: row.display_name,
        cover: LiteModule {
            id: row.cover_id,
            kind: row.cover_kind,
        },
        ending: LiteModule {
            id: row.ending_id,
            kind: row.ending_kind,
        },
        modules: row.modules.into_iter().map(|(id, kind)| LiteModule {
            id, kind
        }).collect(),
        creator_id: row.creator_id,
        author_id: row.author_id,
        publish_at: row.publish_at,
    });

    Ok(jig)
}

pub async fn update(
    pool: &PgPool,
    id: JigId,
    display_name: Option<&str>,
    author_id: Option<Uuid>,
    cover_id: Option<ModuleId>,
    modules: Option<&[ModuleId]>,
    ending_id: Option<ModuleId>,
    publish_at: Option<Option<DateTime<Utc>>>,
) -> anyhow::Result<bool> {
    let mut transaction = pool.begin().await?;
    if !sqlx::query!(
        r#"select exists(select 1 from jig where id = $1) as "exists!""#,
        id.0
    )
    .fetch_one(&mut transaction)
    .await?
    .exists
    {
        return Ok(false);
    }

    if let Some(publish_at) = publish_at {
        sqlx::query!(
            r#"
update jig
set publish_at = $2, updated_at = now()
where id = $1 and $2 is distinct from publish_at"#,
            id.0,
            publish_at
        )
        .execute(&mut transaction)
        .await?;
    }

    sqlx::query!(
        r#"
update jig
set display_name  = coalesce($2, display_name),
    author_id  = coalesce($3, author_id),
    cover_id  = coalesce($4, cover_id),
    ending_id  = coalesce($5, ending_id),
    updated_at  = now()
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id) or
       ($4::uuid is not null and $4 is distinct from cover_id) or
       ($5::uuid is not null and $5 is distinct from ending_id))"#,
        id.0,
        display_name,
        author_id,
        cover_id.map(|it| it.0),
        ending_id.map(|it| it.0)
    )
    .execute(&mut transaction)
    .await?;
    if let Some(module_ids) = modules {
        sqlx::query!("delete from jig_module where jig_id = $1", id.0)
            .execute(&mut transaction)
            .await?;

        for (idx, module_id) in module_ids.iter().enumerate() {
            sqlx::query!(
                r#"insert into jig_module (jig_id, "index", module_id) values ($1, $2, $3)"#,
                id.0,
                idx as i16,
                module_id.0
            )
            .execute(&mut transaction)
            .await?;
        }
    }
    transaction.commit().await?;

    Ok(true)
}

pub async fn delete(pool: &PgPool, id: JigId) -> anyhow::Result<()> {
    sqlx::query!("delete from jig where id = $1", id.0)
        .execute(pool)
        .await
        .map(drop)
        .map_err(Into::into)
}
