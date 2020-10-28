use chrono::{DateTime, Utc};
use shared::domain::jig::{Jig, JigId};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    display_name: &str,
    cover: &serde_json::Value,
    modules: &[serde_json::Value],
    ending: &serde_json::Value,
    creator_id: Uuid,
    publish_at: Option<DateTime<Utc>>,
) -> anyhow::Result<JigId> {
    let mut transaction = pool.begin().await?;
    let jig = sqlx::query!(
        r#"
insert into jig
    (display_name, cover, ending, creator_id, author_id, publish_at)
values ($1, $2, $3, $4, $4, $5)
returning id
"#,
        display_name,
        cover,
        ending,
        creator_id,
        publish_at
    )
    .fetch_one(&mut transaction)
    .await?;

    // todo: batch
    for module in modules {
        sqlx::query!(
            "insert into jig_module (jig_id, module) values ($1, $2)",
            jig.id,
            module
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(JigId(jig.id))
}

pub async fn get(pool: &PgPool, id: JigId) -> anyhow::Result<Option<Jig>> {
    sqlx::query_as!(
        Jig,
        r#"
select 
    id as "id: JigId",
    display_name,
    cover,
    ending,
    creator_id,
    author_id,
    publish_at,
    array(select module from jig_module where jig_id = $1) as "modules!"
from jig
where id = $1
"#,
        id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

pub async fn update(
    pool: &PgPool,
    id: JigId,
    display_name: Option<&str>,
    author_id: Option<Uuid>,
    cover: Option<&serde_json::Value>,
    modules: Option<&[serde_json::Value]>,
    ending: Option<&serde_json::Value>,
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
set display_name        = coalesce($2, display_name),
    author_id  = coalesce($3, author_id),
    cover  = coalesce($4, cover),
    ending  = coalesce($5, ending),
    updated_at  = now()
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id) or
       ($4::jsonb is not null and $4 is distinct from cover) or
       ($5::jsonb is not null and $5 is distinct from ending))"#,
        id.0,
        display_name,
        author_id,
        cover,
        ending
    )
    .execute(&mut transaction)
    .await?;
    if let Some(modules) = modules {
        sqlx::query!("delete from jig_module where jig_id = $1", id.0)
            .execute(&mut transaction)
            .await?;

        for module in modules {
            sqlx::query!(
                "insert into jig_module (jig_id, module) values ($1, $2)",
                id.0,
                module
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
