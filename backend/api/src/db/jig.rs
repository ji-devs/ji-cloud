use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::domain::{
    jig::{module::ModuleId, Jig, JigId, LiteModule, ModuleKind},
    meta::ContentTypeId,
    user::UserScope,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error;

pub async fn create(
    pool: &PgPool,
    display_name: Option<&str>,
    content_types: &[ContentTypeId],
    creator_id: Uuid,
    publish_at: Option<DateTime<Utc>>,
) -> sqlx::Result<JigId> {
    let mut transaction = pool.begin().await?;

    let jig = sqlx::query!(
        r#"
insert into jig
    (display_name, creator_id, author_id, publish_at)
values ($1, $2, $2, $3)
returning id
"#,
        display_name,
        creator_id,
        publish_at
    )
    .fetch_one(&mut transaction)
    .await?;

    super::recycle_metadata(&mut transaction, "jig", jig.id, content_types).await?;

    let default_module_kinds = [Some(ModuleKind::Cover), None];

    // todo: batch
    for (idx, kind) in default_module_kinds.iter().enumerate() {
        sqlx::query!(
            r#"
insert into jig_module (jig_id, "index", kind)
values ($1, $2, $3)"#,
            jig.id,
            idx as i16,
            kind.map(|it| it as i16),
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
    creator_id,
    author_id,
    publish_at,
    array(
        select row (id, kind)
        from jig_module
        where jig_id = $1
        order by "index"
    ) as "modules!: Vec<(ModuleId, Option<ModuleKind>)>",
    array(select row(content_type_id) from jig_content_type where jig_id = $1) as "content_types!: Vec<(ContentTypeId,)>"
from jig
where id = $1"#,
        id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| Jig {
        id: row.id,
        display_name: row.display_name,
        modules: row.modules.into_iter().map(|(id, kind,)| LiteModule {
            id, kind
        }).collect(),
        content_types: row.content_types.into_iter().map(|(it,)| it).collect(),
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
    content_types: Option<&[ContentTypeId]>,
    publish_at: Option<Option<DateTime<Utc>>>,
) -> Result<(), error::UpdateWithMetadata> {
    let mut transaction = pool.begin().await?;
    if !sqlx::query!(
        r#"select exists(select 1 from jig where id = $1 for update) as "exists!""#,
        id.0
    )
    .fetch_one(&mut transaction)
    .await?
    .exists
    {
        return Err(error::UpdateWithMetadata::ResourceNotFound);
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
    updated_at  = now()
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id))"#,
        id.0,
        display_name,
        author_id
    )
    .execute(&mut transaction)
    .await?;

    if let Some(content_types) = content_types {
        super::recycle_metadata(&mut transaction, "jig", id.0, content_types)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: JigId) -> anyhow::Result<()> {
    sqlx::query!("delete from jig where id = $1", id.0)
        .execute(pool)
        .await
        .map(drop)
        .map_err(Into::into)
}

pub async fn list(
    pool: &sqlx::Pool<sqlx::Postgres>,
    is_published: Option<bool>,
    author_id: Option<Uuid>,
    page: i32,
) -> sqlx::Result<Vec<Jig>> {
    sqlx::query!(
        r#"
select  
    id as "id: JigId",
    display_name,
    creator_id,
    author_id,
    publish_at,
    array(
        select row (id, kind)
        from jig_module
        where jig_id = jig.id
        order by "index"
    ) as "modules!: Vec<(ModuleId, Option<ModuleKind>)>",
    array(select row(content_type_id) from jig_content_type where jig_id = jig.id) as "content_types!: Vec<(ContentTypeId,)>"
from jig
where 
    publish_at < now() is not distinct from $1 or $1 is null
    and author_id is not distinct from $3 or $3 is null
order by coalesce(updated_at, created_at) desc
limit 20 offset 20 * $2
"#, is_published,
page,
author_id,

    )
    .fetch(pool)
    .map_ok(|row| Jig {
        id: row.id,
        display_name: row.display_name,
        modules: row.modules.into_iter().map(|(id, kind)| LiteModule {
             id, kind
        }).collect(),
        content_types: row.content_types.into_iter().map(|(it,)| it).collect(),
        creator_id: row.creator_id,
        author_id: row.author_id,
        publish_at: row.publish_at,
    })
    .try_collect()
    .await
}

pub async fn filtered_count(
    db: &PgPool,
    is_published: Option<bool>,
    author_id: Option<Uuid>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        r#"
select count(*) as "count!: i64"
from jig
where
    publish_at < now() is not distinct from $1 or $1 is null
    and author_id is not distinct from $2 or $2 is null
"#,
        is_published,
        author_id,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn clone(db: &PgPool, parent: JigId, user_id: Uuid) -> sqlx::Result<Option<JigId>> {
    let mut txn = db.begin().await?;

    let new_id = sqlx::query!(
        r#"
insert into jig (display_name, parents, creator_id, author_id)
select display_name, array_append(parents, id), $2 as creator_id, $2 as author_id from jig where id = $1
returning id
"#,
        parent.0, user_id
    )
    .fetch_optional(&mut txn)
    .await?;

    let new_id = match new_id {
        Some(it) => it.id,
        None => return Ok(None),
    };

    sqlx::query!(
        r#"
insert into jig_module ("index", jig_id, kind, contents)
select "index", $2 as "jig_id", kind, contents
from jig_module where jig_id = $1
"#,
        parent.0,
        new_id
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(Some(JigId(new_id)))
}

pub async fn authz(db: &PgPool, user_id: Uuid, jig_id: Option<JigId>) -> Result<(), error::Auth> {
    let authed = match jig_id {
        None => {
            sqlx::query!(
                r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
                user_id,
                &[
                    UserScope::Admin as i16,
                    UserScope::AdminJig as i16,
                    UserScope::ManageSelfJig as i16,
                ][..],
            )
            .fetch_one(db)
            .await?
            .authed
        }
        Some(id) => {
            sqlx::query!(
                r#"
select exists (
    select 1 from user_scope where user_id = $1 and scope = any($2)
) or (
    exists (select 1 from user_scope where user_id = $1 and scope = $3) and
    not exists (select 1 from jig where jig.id = $4 and jig.author_id <> $1)
) as "authed!"
"#,
                user_id,
                &[UserScope::Admin as i16, UserScope::AdminJig as i16,][..],
                UserScope::ManageSelfJig as i16,
                id.0
            )
            .fetch_one(db)
            .await?
            .authed
        }
    };

    if !authed {
        return Err(error::Auth::Forbidden);
    }

    Ok(())
}
