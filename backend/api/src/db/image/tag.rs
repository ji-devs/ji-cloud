use crate::error;
use paperclip::actix::NoContent;
use shared::domain::{image::tag::ImageTagResponse, meta::TagId};
use sqlx::postgres::PgDatabaseError;
use sqlx::PgPool;

pub async fn list(db: &PgPool) -> sqlx::Result<Vec<ImageTagResponse>> {
    sqlx::query_as!(
        ImageTagResponse,
        r#"
select id as "id: TagId", display_name, index from "image_tag"
order by index
            "#
    )
    .fetch_all(db)
    .await
}

pub async fn create(
    db: &PgPool,
    index: i16,
    display_name: &str,
) -> Result<(i16, String, TagId), error::Tag> {
    let mut txn = db.begin().await?;

    let res = sqlx::query!(
        // language=SQL
        r#"
insert into image_tag (index, display_name)
values ($1, $2)
returning id as "id: TagId", index as "index: i16", display_name
            "#,
        index,
        display_name,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await.map_err(|err| match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("image_tag_index_key") =>
        {
            error::Tag::TakenIndex
        }
        e => e.into(),
    })?;

    Ok((res.index, res.display_name, res.id))
}

pub async fn update(
    db: &PgPool,
    curr_index: i16,
    display_name: Option<&str>,
    new_index: Option<i16>,
) -> Result<NoContent, error::Tag> {
    let mut txn = db.begin().await?;

    let res = sqlx::query!(
        r#"select index as "index: i16" from image_tag where index = $1 for update"#,
        curr_index
    )
    .fetch_optional(&mut txn)
    .await?;

    if res.is_none() {
        txn.commit().await?;
        return Err(error::Tag::ResourceNotFound);
    }

    if let Some(display_name) = display_name {
        sqlx::query!(
            r#"update image_tag set display_name = $2 where index = $1"#,
            curr_index,
            display_name,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(new_index) = new_index {
        sqlx::query!(
            r#"
            update image_tag set index = $2 where index = $1
            "#,
            curr_index,
            new_index
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await.map_err(|err| match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("image_tag_index_key") =>
        {
            error::Tag::TakenIndex
        }
        e => e.into(),
    })?;

    Ok(NoContent)
}

pub async fn delete(db: &PgPool, index: i16) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
delete from image_tag where index = $1
            "#,
        index
    )
    .execute(db)
    .await?;

    Ok(())
}
