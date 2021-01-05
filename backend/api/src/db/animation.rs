use shared::{domain::animation::AnimationId, media::AnimationVariant};
use sqlx::PgPool;

pub async fn delete(db: &PgPool, animation: AnimationId) -> sqlx::Result<Option<AnimationVariant>> {
    let mut conn = db.begin().await?;

    let res = sqlx::query!(
        r#"delete from animation where id = $1 returning variant as "variant: AnimationVariant""#,
        animation.0
    )
    .fetch_optional(&mut conn)
    .await?
    .map(|it| it.variant);
    conn.commit().await?;

    Ok(res)
}
