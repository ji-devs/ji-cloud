use shared::domain::{
    circle::{Circle, CircleId},
    image::ImageId,
    user::UserScope,
};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::error;

pub async fn create(
    conn: &mut PgConnection,
    display_name: &str,
    description: &str,
    image: Option<ImageId>,
    creator_id: Uuid,
) -> sqlx::Result<CircleId> {
    let image_id = if let Some(id) = image {
        Some(id.0)
    } else {
        None
    };

    let id: CircleId = sqlx::query!(
        r#"
insert into circle (display_name, description, image, creator_id) values ($1, $2, $3, $4)
returning id as "id: CircleId"
        "#,
        display_name,
        description,
        image_id,
        creator_id,
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    Ok(id)
}

pub async fn update(
    pool: &PgPool,
    id: CircleId,
    display_name: Option<&str>,
    description: Option<&str>,
    image: Option<ImageId>,
) -> anyhow::Result<bool> {
    let mut txn = pool.begin().await?;

    if !sqlx::query!(
        r#"select exists(select 1 from circle where id = $1) as "exists!""#,
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
update circle
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
update circle
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

    if let Some(image) = image {
        sqlx::query!(
            r#"
update circle
set image = $2,
    updated_at = now()
where id = $1 and $2 is distinct from image"#,
            id.0,
            image.0,
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(true)
}

pub async fn delete(db: &PgPool, id: CircleId) -> sqlx::Result<()> {
    let mut conn = db.begin().await?;

    sqlx::query!("delete from circle where id = $1", id.0)
        .execute(&mut conn)
        .await?;

    conn.commit().await
}

pub async fn get_one(db: &PgPool, id: CircleId) -> anyhow::Result<Option<Circle>> {
    let res = sqlx::query!(
        r#"
select id            as "circle_id: CircleId",
       display_name,
       description,
       image         as "image?: ImageId",
       member_count,
       creator_id,
       created_at,
       updated_at
from circle
where id = $1
"#,
        id.0
    )
    .fetch_optional(db)
    .await?;

    let circle = res.map(|row| Circle {
        id: row.circle_id,
        display_name: row.display_name,
        created_by: row.creator_id,
        description: row.description,
        image: row.image,
        member_count: row.member_count as u32,
        created_at: row.created_at,
        last_edited: row.updated_at,
    });

    Ok(circle)
}

pub async fn join_circle(db: &PgPool, user_id: Uuid, id: CircleId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
insert into circle_member(id, user_id) values($1, $2)
"#,
        id.0,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("User is already a member of this circle"))?;

    Ok(())
}

pub async fn leave_circle(db: &PgPool, user_id: Uuid, id: CircleId) -> anyhow::Result<()> {
    sqlx::query!(
        "delete from circle_member where id = $1 and user_id = $2",
        id.0,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("User is not part of circle"))?;

    Ok(())
}

pub async fn browse(
    db: &PgPool,
    creator_id: Option<Uuid>,
    page_limit: u32,
    page: i32,
) -> sqlx::Result<Vec<Circle>> {
    let mut txn = db.begin().await?;

    let circle_data = sqlx::query!(
        r#"
        with cte as (
            select array(select id  as "id!"
            from "circle"
            where creator_id = $1 or $1 is null
            order by coalesce(updated_at, created_at)) as id
        ),
        cte1 as (
            select * from unnest((select distinct id from cte)) with ordinality t(id
           , ord) order by ord
        )
        select  circle.id            as "circle_id!: CircleId",
                display_name        as "display_name!",
                description         as "description!",
                image               as "image?: ImageId",
                member_count        as "member_count!",
                creator_id          as "creator_id!",
                created_at,
                updated_at   
        from "circle"
            inner join cte1 on cte1.id = "circle".id
            where ord > (1 * $2 * $3)
            order by ord 
            limit $3
            "#,
        creator_id,
        page as i32,
        page_limit as i32,
    )
    .fetch_all(&mut txn)
    .await?;

    let res = circle_data
        .into_iter()
        .map(|row| Circle {
            id: row.circle_id,
            display_name: row.display_name,
            created_by: row.creator_id,
            description: row.description,
            image: row.image,
            member_count: row.member_count as u32,
            created_at: row.created_at,
            last_edited: row.updated_at,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Circle>> {
    let mut txn = db.begin().await?;

    let res: Vec<_> = sqlx::query!(
        //language=SQL
        r#"
select  id            as "circle_id!: CircleId",
        display_name  as "display_name!",
        description   as "description!",
        image         as "image?: ImageId",
        member_count  as "member_count!: u32",
        creator_id    as "creator_id!",
        created_at    as "created_at!",
        updated_at   
from circle
inner join unnest($1::uuid[])
with ordinality t(id, ord) using (id)
"#,
        ids
    )
    .fetch_all(&mut txn)
    .await?;

    let v = res
        .into_iter()
        .map(|row| Circle {
            id: row.circle_id,
            display_name: row.display_name,
            created_by: row.creator_id,
            description: row.description,
            image: row.image,
            member_count: row.member_count,
            created_at: row.created_at,
            last_edited: row.updated_at,
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn browse_circle_members(db: &PgPool, id: CircleId) -> anyhow::Result<Vec<Uuid>> {
    let mut txn = db.begin().await?;

    let res = sqlx::query!(
        //language=SQL
        r#"
select user_id  
from circle_member
where id = $1
"#,
        id.0
    )
    .fetch_all(&mut txn)
    .await?;

    txn.rollback().await?;

    Ok(res.into_iter().map(|row| row.user_id).collect())
}

pub async fn valid_circle(db: &PgPool, id: CircleId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
select exists(select 1 from circle where id = $1) as "valid!"
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
    circle_id: Option<CircleId>,
) -> Result<(), error::Auth> {
    let authed = match circle_id {
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
    not exists (select 1 from circle where id = $4 and circle.creator_id <> $1)
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
    let circle = sqlx::query!(
        //language=SQL
        r#"
select count(distinct id) as "count!: i64"
    from circle
    where creator_id = $1 or $1 is null
"#,
        creator_id,
    )
    .fetch_one(db)
    .await?;

    Ok(circle.count as u64)
}