use chrono::{DateTime, Utc};
use shared::domain::animation::{AnimationId, AnimationKind, AnimationMetadata};
use sqlx::{PgConnection, PgPool};

pub async fn delete(db: &PgPool, animation: AnimationId) -> sqlx::Result<Option<AnimationKind>> {
    let mut conn = db.begin().await?;

    let res = sqlx::query!(
        r#"delete from animation_metadata where id = $1 returning kind as "kind: AnimationKind""#,
        animation.0
    )
    .fetch_optional(&mut conn)
    .await?
    .map(|it| it.kind);
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
    kind: AnimationKind,
) -> sqlx::Result<AnimationId> {
    let id: AnimationId = sqlx::query!(
        r#"
insert into animation_metadata (name, description, is_premium, publish_at, kind, looping) values ($1, $2, $3, $4, $5, $6)
returning id as "id: AnimationId"
        "#,
        name,
        description,
        is_premium,
        publish_at,
        kind as i16,
        is_looping,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

pub async fn get_one(db: &PgPool, id: AnimationId) -> sqlx::Result<Option<AnimationMetadata>> {
    sqlx::query_as(
        r#"
select  id,
        name,
        description,
        is_premium,
        publish_at,
        created_at,
        updated_at,
        kind,
        looping         as is_looping,
        array((select row (style_id) from animation_style where animation_id = animation_metadata.id)) as styles
from animation_metadata inner join global_animation_upload gau on animation_id=id
where id = $1 and processing_result is true
"#)
    .bind(id)
    .fetch_optional(db)
    .await
}
