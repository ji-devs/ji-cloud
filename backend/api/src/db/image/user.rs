use futures::stream::BoxStream;
use shared::domain::image::{user::UserImage, ImageId};
use sqlx::PgPool;

pub async fn create(conn: &PgPool) -> sqlx::Result<ImageId> {
    let id: ImageId = sqlx::query!(
        r#"
insert into user_image_library default values
returning id as "id: ImageId"
"#,
    )
    .fetch_one(conn)
    .await?
    .id;

    Ok(id)
}

pub async fn delete(db: &PgPool, image: ImageId) -> sqlx::Result<()> {
    sqlx::query!("delete from user_image_library where id = $1", image.0)
        .execute(db)
        .await
        .map(drop)
}

pub async fn get(db: &PgPool, image: ImageId) -> sqlx::Result<Option<UserImage>> {
    sqlx::query_as!(
        UserImage,
        r#"select id as "id: ImageId" from user_image_library where id = $1"#,
        image.0
    )
    .fetch_optional(db)
    .await
}

pub fn list(db: &PgPool) -> BoxStream<'_, sqlx::Result<UserImage>> {
    sqlx::query_as!(
        UserImage,
        r#"select id as "id: ImageId" from user_image_library order by created_at desc"#,
    )
    .fetch(db)
}
