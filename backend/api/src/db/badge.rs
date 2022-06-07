use shared::domain::{
    badge::{Badge, BadgeId},
    user::UserScope,
};
use sqlx::{PgConnection, PgPool};
use url::Url;
use uuid::Uuid;

use crate::error;

pub async fn create(
    conn: &mut PgConnection,
    display_name: &str,
    description: &str,
    thumbnail: url::Url,
    creator_id: Uuid,
) -> sqlx::Result<BadgeId> {
    let id: BadgeId = sqlx::query!(
        r#"
insert into badge (display_name, description, thumbnail, creator_id) values ($1, $2, $3, $4)
returning id as "id: BadgeId"
        "#,
        display_name,
        description,
        thumbnail.as_str(),
        creator_id,
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    Ok(id)
}

pub async fn update(
    pool: &PgPool,
    id: BadgeId,
    display_name: Option<&str>,
    description: Option<&str>,
    thumbnail: Option<url::Url>,
) -> anyhow::Result<bool> {
    let mut txn = pool.begin().await?;

    if !sqlx::query!(
        r#"select exists(select 1 from badge where id = $1) as "exists!""#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?
    .exists
    {
        return Ok(false);
    }

    if let Some(display_name) = display_name {
        sqlx::query!(
            r#"
update badge
set display_name = $2,
    updated_at = now()
where id = $1 and $2 is distinct from display_name
"#,
            id.0,
            display_name,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(description) = description {
        sqlx::query!(
            r#"
update badge
set description = $2,
    updated_at = now()
where id = $1 and $2 is distinct from description
"#,
            id.0,
            description,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(thumbnail) = thumbnail {
        sqlx::query!(
            r#"
update badge
set thumbnail = $2,
    updated_at = now()
where id = $1 and $2 is distinct from thumbnail"#,
            id.0,
            thumbnail.as_str(),
        )
        .execute(&mut txn)
        .await?;
    }

    Ok(true)
}

pub async fn delete(db: &PgPool, id: BadgeId) -> sqlx::Result<()> {
    let mut conn = db.begin().await?;

    sqlx::query!("delete from badge where id = $1", id.0)
        .execute(&mut conn)
        .await?;

    conn.commit().await
}

pub async fn get_one(db: &PgPool, id: BadgeId) -> anyhow::Result<Option<Badge>> {
    let res = sqlx::query!(
        r#"
select id            as "badge_id: BadgeId",
       display_name,
       description,
       thumbnail,
       member_count,
       creator_id
from badge
where id = $1
"#,
        id.0
    )
    .fetch_optional(db)
    .await?;

    let badge = res.map(|row| Badge {
        id: row.badge_id,
        display_name: row.display_name,
        created_by: row.creator_id,
        description: row.description,
        thumbnail: Url::parse(&row.thumbnail).unwrap(),
        member_count: row.member_count as u32,
    });

    Ok(badge)
}

pub async fn join_badge(db: &PgPool, user_id: Uuid, id: BadgeId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
insert into badge_member(id, user_id) values($1, $2)
"#,
        id.0,
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|_| anyhow::anyhow!("User is already a member of this badge"))?;

    Ok(())
}

pub async fn leave_badge(db: &PgPool, user_id: Uuid, id: BadgeId) -> anyhow::Result<()> {
    sqlx::query!(
        "delete from badge_member where id = $1 and user_id = $2",
        id.0,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("User is not part of badge"))?;

    Ok(())
}

pub async fn browse(
    db: &PgPool,
    creator_id: Option<Uuid>,
    page_limit: u32,
    page: i32,
) -> sqlx::Result<Vec<Badge>> {
    let res: Vec<_> = sqlx::query!(
        //language=SQL
        r#"
select  id            as "badge_id!: BadgeId",
        display_name,
        description,
        thumbnail,
        member_count  as "member_count!: u32",
        creator_id
from badge
where (creator_id = $1 or $1 is null)
order by coalesce(updated_at, created_at)
offset $2
limit $3
"#,
        creator_id,
        page as i64,
        page_limit as i32,
    )
    .fetch_all(db)
    .await?;

    Ok(res
        .into_iter()
        .map(|row| Badge {
            id: row.badge_id,
            display_name: row.display_name,
            created_by: row.creator_id,
            description: row.description,
            thumbnail: Url::parse(&row.thumbnail).unwrap(),
            member_count: row.member_count,
        })
        .collect())
}

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Badge>> {
    let mut txn = db.begin().await?;

    let res: Vec<_> = sqlx::query!(
        //language=SQL
        r#"
select  id            as "badge_id!: BadgeId",
        display_name   as "display_name!",
        description     as "description!",
        thumbnail       as "thumbnail!",
        member_count  as "member_count!: u32",
        creator_id    as "creator_id!"
from badge
inner join unnest($1::uuid[])
with ordinality t(id, ord) using (id)
"#,
        ids
    )
    .fetch_all(&mut txn)
    .await?;

    let v = res
        .into_iter()
        .map(|row| Badge {
            id: row.badge_id,
            display_name: row.display_name,
            created_by: row.creator_id,
            description: row.description,
            thumbnail: Url::parse(&row.thumbnail).unwrap(),
            member_count: row.member_count,
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn browse_badge_members(db: &PgPool, id: BadgeId) -> anyhow::Result<Vec<Uuid>> {
    let mut txn = db.begin().await?;

    let res = sqlx::query!(
        //language=SQL
        r#"
select user_id  
from badge_member
where id = $1
"#,
        id.0
    )
    .fetch_all(&mut txn)
    .await?;

    txn.rollback().await?;

    Ok(res.into_iter().map(|row| row.user_id).collect())
}

pub async fn valid_badge(db: &PgPool, id: BadgeId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
select exists(select 1 from badge where id = $1) as "valid!"
"#,
        id.0
    )
    .fetch_one(db)
    .await?;

    Ok(())
}

pub async fn authz(
    db: &PgPool,
    user_id: Uuid,
    badge_id: Option<BadgeId>,
) -> Result<(), error::Auth> {
    let authed = match badge_id {
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
                //language=SQL
                r#"
select exists (
    select 1 from user_scope where user_id = $1 and scope = any($2)
) or (
    exists (select 1 from user_scope where user_id = $1 and scope = $3) and
    not exists (select 1 from badge where id = $4 and badge.creator_id <> $1)
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

pub async fn filtered_count(db: &PgPool, creator_id: Option<Uuid>) -> sqlx::Result<u64> {
    let badge = sqlx::query!(
        //language=SQL
        r#"
select count(distinct id) as "count!: i64"
    from badge
    where creator_id = $1 or $1 is null
"#,
        creator_id,
    )
    .fetch_one(db)
    .await?;

    Ok(badge.count as u64)
}
