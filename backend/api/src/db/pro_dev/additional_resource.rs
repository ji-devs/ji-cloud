use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use shared::domain::{
    additional_resource::{AdditionalResourceId, ResourceContent},
    asset::DraftOrLive,
    audio::AudioId,
    image::ImageId,
    meta::ResourceTypeId,
    pdf::PdfId,
    pro_dev::ProDevId,
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
    pro_dev_id: ProDevId,
    display_name: String,
    resource_type_id: ResourceTypeId,
    resource_content: ResourceContent,
) -> anyhow::Result<AdditionalResourceId> {
    // Checks if Audio and Image IDs exists
    let resource: serde_json::Value = check_content(pool, resource_content).await?;

    sqlx::query!(
        r#"
insert into pro_dev_data_resource (pro_dev_data_id, resource_type_id, resource_content, display_name)
values ((select draft_id from pro_dev where id = $1), $2, $3, $4)
returning id as "id!: AdditionalResourceId"
        "#,
        pro_dev_id.0,
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
    pro_dev_id: ProDevId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
) -> anyhow::Result<(String, ResourceTypeId, ResourceContent), error::NotFound> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, pro_dev_id)
        .await
        .ok_or(error::NotFound::ResourceNotFound)?;

    let pro_dev_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    if !sqlx::query!(
        //language=SQL
        r#"
select exists(select 1 from pro_dev_data_resource "pddr" where pro_dev_data_id = $1
    and pddr.id = $2) as "exists!"
    "#,
        pro_dev_data_id,
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
from pro_dev_data_resource "pddr"
where pro_dev_data_id = $1
  and pddr.id = $2
        "#,
        pro_dev_data_id,
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
    pro_dev_id: ProDevId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
    display_name: Option<String>,
    resource_type_id: Option<ResourceTypeId>,
    resource_content: Option<ResourceContent>,
) -> anyhow::Result<(), error::Auth> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, pro_dev_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get pro_dev_data IDs"))?;

    let pro_dev_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    if let Some(display_name) = display_name {
        sqlx::query!(
            //language=SQL
            r#"
update pro_dev_data_resource
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
update pro_dev_data_resource
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
update pro_dev_data_resource
set resource_content = $3
where pro_dev_data_id = $1 and id = $2
            "#,
            pro_dev_data_id,
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
    pro_dev_id: ProDevId,
    id: AdditionalResourceId,
) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, pro_dev_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get pro_dev_data IDs"))?;

    sqlx::query!(
        //language=SQL
        r#"
delete
from pro_dev_data_resource
where pro_dev_data_id = $1
   or pro_dev_data_id = $2
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
