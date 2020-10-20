use chrono::{DateTime, Utc};
use shared::domain::jig::{Jig, JigId, ModuleId};
use sqlx::PgPool;
use uuid::Uuid;

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
    let jig = sqlx::query!(
        r#"
insert into jig
    (display_name, cover_id, ending_id, creator_id, author_id, publish_at)
values ($1, $2, $3, $4, $4, $5)
returning id
"#,
        display_name,
        cover_id.map(|it| it.0),
        ending_id.map(|it| it.0),
        creator_id,
        publish_at
    )
    .fetch_one(&mut transaction)
    .await?;

    // todo: batch
    for (idx, module_id) in module_ids.iter().enumerate() {
        sqlx::query!(
            r#"insert into jig_module (jig_id, "index", module_id) values ($1, $2, $3)"#,
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
select 
    id as "id: JigId",
    display_name,
    cover_id,
    ending_id,
    creator_id,
    author_id,
    publish_at,
    array(select module_id from jig_module where jig_id = $1 order by "index") as "module_ids!"
from jig
where id = $1
"#,
        id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| Jig {
        id: row.id,
        display_name: row.display_name,
        cover_id: row.cover_id.map(ModuleId),
        ending_id: row.ending_id.map(ModuleId),
        module_ids: row.module_ids.into_iter().map(ModuleId).collect(),
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
