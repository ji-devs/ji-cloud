use crate::error;
use shared::domain::{image::tag::ImageTagResponse, meta::ImageTagIndex};
use sqlx::{postgres::PgDatabaseError, PgPool};

pub fn handle_tag_err(err: sqlx::Error) -> error::Tag {
    match err {
        sqlx::Error::Database(err)
            if err.downcast_ref::<PgDatabaseError>().constraint()
                == Some("image_tag_index_pkey") =>
        {
            error::Tag::TakenIndex
        }
        e => e.into(),
    }
}

pub async fn list(db: &PgPool) -> sqlx::Result<Vec<ImageTagResponse>> {
    sqlx::query_as!(
        ImageTagResponse,
        r#"
select index as "index: ImageTagIndex", display_name from "image_tag"
order by index
            "#
    )
    .fetch_all(db)
    .await
}

pub async fn create(
    db: &PgPool,
    index: ImageTagIndex,
    display_name: &str,
) -> Result<(ImageTagIndex, String), error::Tag> {
    let mut txn = db.begin().await?;

    let res = sqlx::query!(
        // language=SQL
        r#"
insert into image_tag (index, display_name)
values ($1, $2)
returning index as "index: ImageTagIndex", display_name
            "#,
        index.0,
        display_name,
    )
    .fetch_one(&mut txn)
    .await
    .map_err(handle_tag_err)?;

    txn.commit().await?;

    Ok((res.index, res.display_name))
}

pub async fn update(
    db: &PgPool,
    curr_index: i16,
    display_name: Option<&str>,
    new_index: Option<i16>,
) -> Result<(), error::Tag> {
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
        .await
        .map_err(handle_tag_err)?;
    }

    txn.commit().await?;

    Ok(())
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
