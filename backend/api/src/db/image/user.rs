use futures::stream::BoxStream;
use shared::domain::image::{user::UserImage, ImageId, ImageKind};
use sqlx::{PgConnection, PgPool};
use tracing::{instrument, Instrument};
use uuid::Uuid;

#[instrument(skip(pool))]
pub async fn create(pool: &PgPool, user_id: &Uuid, kind: ImageKind) -> sqlx::Result<ImageId> {
    let mut txn = pool.begin().await?;
    let id: ImageId = sqlx::query!(
        //language=SQL
        r#"
insert into user_image_library (user_id, kind)
values ($1, $2)
returning id as "id: ImageId"
"#,
        user_id,
        kind as i16,
    )
    .fetch_one(&mut txn)
    .instrument(tracing::info_span!("inser user_image_library"))
    .await?
    .id;

    sqlx::query!("insert into user_image_upload (image_id) values ($1)", id.0)
        .execute(&mut txn)
        .instrument(tracing::info_span!("inser user_image_upload"))
        .await?;

    txn.commit().await?;

    Ok(id)
}

pub async fn delete(db: &PgPool, user_id: Uuid, image_id: ImageId) -> sqlx::Result<()> {
    sqlx::query!(
        "delete from user_image_library where user_id = $1 and id = $2",
        user_id,
        image_id.0
    )
    .execute(db)
    .await
    .map(drop)
}

pub async fn get(db: &PgPool, user_id: Uuid, image_id: ImageId) -> sqlx::Result<Option<UserImage>> {
    sqlx::query_as!(
        UserImage,
        // language=SQL
        r#"
select id as "id: ImageId", kind as "kind: ImageKind"
from user_image_library
         inner join user_image_upload
                    on user_image_library.id = user_image_upload.image_id
where user_id = $1
  and id = $2
  and processing_result is true
        "#,
        user_id,
        image_id.0,
    )
    .fetch_optional(db)
    .await
}

pub fn list(
    db: &PgPool,
    user_id: Uuid,
    kind: Option<ImageKind>,
) -> BoxStream<'_, sqlx::Result<UserImage>> {
    sqlx::query_as!(
        UserImage,
        // language=SQL
        r#"
select id as "id: ImageId", kind as "kind: ImageKind"
from user_image_library
         join user_image_upload
              on user_image_library.id = user_image_upload.image_id
where processing_result is true
  and user_id = $1
  and (kind is not distinct from $2 or $2 is null)
order by created_at desc
"#,
        user_id,
        kind.map(|it| it as i16)
    )
    .fetch(db)
}

/// checks if the user owns the image requested.
///
/// Returns ResourceNotFound even if the image exists but the user does not h
pub async fn auth_user_image(
    txn: &mut PgConnection,
    user_id: &Uuid,
    image_id: &ImageId,
) -> anyhow::Result<(), crate::error::Upload> {
    let exists = sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from user_image_library where user_id = $1 and id = $2) as "exists!"
    "#,
        user_id,
        image_id.0
    )
    .fetch_one(txn)
    .await?
    .exists;

    if !exists {
        return Err(crate::error::Upload::ResourceNotFound);
    }

    Ok(())
}
