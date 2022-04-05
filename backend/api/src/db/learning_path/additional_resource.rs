use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use shared::domain::{
    audio::AudioId,
    image::ImageId,
    jig::DraftOrLive,
    learning_path::additional_resource::{AdditionalResourceId, ResourceContent},
    learning_path::LearningPathId,
    meta::ResourceTypeId,
    pdf::PdfId,
};
use sqlx::PgPool;
use url::Url;

use crate::error;

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ResourceObject {
    content: serde_json::Value,
}

pub async fn create(
    pool: &PgPool,
    learning_path_id: LearningPathId,
    display_name: String,
    resource_type_id: ResourceTypeId,
    resource_content: ResourceContent,
) -> anyhow::Result<AdditionalResourceId> {
    // Checks if Audio and Image IDs exists
    let resource: serde_json::Value = check_content(pool, resource_content).await?;

    sqlx::query!(
        r#"
insert into learning_path_data_resource (learning_path_data_id, resource_type_id, resource_content, display_name)
values ((select draft_id from learning_path where id = $1), $2, $3, $4)
returning id as "id!: AdditionalResourceId"
        "#,
        learning_path_id.0,
        resource_type_id.0,
        resource,
        display_name
    )
    .fetch_one(pool)
    .await
    .map(|it| it.id)
    .map_err(Into::into)
}

pub async fn get(
    pool: &PgPool,
    learning_path_id: LearningPathId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
) -> anyhow::Result<(String, ResourceTypeId, ResourceContent), error::NotFound> {
    let mut txn = pool.begin().await?;

    log::warn!("Before get_draft_and_live_ids: {:?}", learning_path_id);

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, learning_path_id)
        .await
        .ok_or(error::NotFound::ResourceNotFound)?;

    log::warn!("After draft_live: {:?}, live_ids: {:?}", draft_id, live_id);

    let learning_path_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    log::warn!("After live_ids: {:?}", learning_path_data_id);

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from learning_path_data_resource "jdar" where learning_path_data_id = $1
    and jdar.id = $2) as "exists!"
    "#,
        learning_path_data_id,
        id.0,
    )
    .fetch_one(&mut txn)
    .await?
    .exists
    {
        return Err(error::NotFound::ResourceNotFound);
    }

    let res = sqlx::query!(
        r#"
select display_name         as "display_name!",
       resource_type_id     as "resource_type_id!: ResourceTypeId",
       resource_content    as "resource_content!"
from learning_path_data_resource "jdar"
where learning_path_data_id = $1
  and jdar.id = $2
        "#,
        learning_path_data_id,
        id.0,
    )
    .fetch_one(&mut txn)
    .await?;

    let content: ResourceContent = serde_json::from_value::<ResourceContent>(res.resource_content)?;

    txn.rollback().await?;

    Ok((res.display_name, res.resource_type_id, content))
}

pub async fn update(
    pool: &PgPool,
    learning_path_id: LearningPathId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
    display_name: Option<String>,
    resource_type_id: Option<ResourceTypeId>,
    resource_content: Option<ResourceContent>,
) -> anyhow::Result<(), error::Auth> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, learning_path_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get learning_path_data IDs"))?;

    let learning_path_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    if let Some(display_name) = display_name {
        sqlx::query!(
            //language=SQL
            r#"
update learning_path_data_resource
set display_name = coalesce($2, display_name)
where id = $1 and $2 is distinct from display_name
            "#,
            id.0,
            display_name
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(resource_type_id) = resource_type_id {
        sqlx::query!(
            //language=SQL
            r#"
update learning_path_data_resource
set resource_type_id = coalesce($2, resource_type_id)
where id = $1 and $2 is distinct from resource_type_id
            "#,
            id.0,
            resource_type_id.0
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(resource_content) = resource_content {
        sqlx::query!(
            //language=SQL
            r#"
update learning_path_data_resource
set resource_content = $3
where learning_path_data_id = $1 and id = $2
            "#,
            learning_path_data_id,
            id.0,
            json!(resource_content)
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(
    pool: &PgPool,
    learning_path_id: LearningPathId,
    id: AdditionalResourceId,
) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, learning_path_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get learning_path_data IDs"))?;

    sqlx::query!(
        //language=SQL
        r#"
delete
from learning_path_data_resource
where learning_path_data_id = $1
   or learning_path_data_id = $2
    and id = $3
        "#,
        draft_id,
        live_id,
        id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn check_content(db: &PgPool, content: ResourceContent) -> anyhow::Result<Value> {
    let resource: serde_json::Value = match content {
        ResourceContent::ImageId(data) => {
            sqlx::query!(
                r#"select id as "id: ImageId" from user_image_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Image Id does not exist"))?;

            json!(ResourceContent::ImageId(data))
        }
        ResourceContent::AudioId(data) => {
            sqlx::query!(
                r#"select id as "id: AudioId" from user_audio_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Audio Id does not exist"))?;

            json!(ResourceContent::AudioId(data))
        }
        ResourceContent::Link(data) => {
            let data = Url::parse(data.as_str())?;

            json!(ResourceContent::Link(data))
        }
        ResourceContent::PdfId(data) => {
            sqlx::query!(
                r#"select id as "id: PdfId" from user_pdf_library where id = $1"#,
                data.0
            )
            .fetch_one(db)
            .await
            .map_err(|_| anyhow::anyhow!("Pdf Id does not exist"))?;

            json!(ResourceContent::PdfId(data))
        }
    };

    Ok(resource)
}
