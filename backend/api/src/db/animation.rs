use chrono::{DateTime, Utc};
use shared::domain::animation::{AnimationId, AnimationKind, AnimationMetadata};
use sqlx::{PgConnection, PgPool};

pub async fn delete(db: &PgPool, animation: AnimationId) -> sqlx::Result<Option<AnimationKind>> {
    let mut conn = db.begin().await?;

    let res = sqlx::query!(
        r#"delete from animation where id = $1 returning variant as "variant: AnimationKind""#,
        animation.0
    )
    .fetch_optional(&mut conn)
    .await?
    .map(|it| it.variant);
    conn.commit().await?;

    Ok(res)
}

pub async fn create(
    conn: &mut PgConnection,
    name: &str,
    description: &str,
    is_premium: bool,
    is_looping: bool,
    publish_at: Option<DateTime<Utc>>,
    variant: AnimationKind,
) -> sqlx::Result<AnimationId> {
    let id: AnimationId = sqlx::query!(
        r#"
insert into animation (name, description, is_premium, publish_at, variant, looping) values ($1, $2, $3, $4, $5, $6)
returning id as "id: AnimationId"
        "#,
        name,
        description,
        is_premium,
        publish_at,
        variant as i16,
        is_looping,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

pub async fn get_one(db: &PgPool, id: AnimationId) -> sqlx::Result<Option<AnimationMetadata>> {
    sqlx::query_as!(
        AnimationMetadata,
        r#"
select id as "id: AnimationId",
       name,
       description,
       is_premium,
       publish_at,
       created_at,
       updated_at,
       variant as "kind: AnimationKind",
       looping as is_looping
from animation
where id = $1
"#,
        id.0
    )
    .fetch_optional(db)
    .await
}
