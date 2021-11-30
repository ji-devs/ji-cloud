use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use shared::domain::{
    audio::AudioId,
    image::ImageId,
    jig::{
        additional_resource::{AdditionalResourceId, ResourceContent},
        DraftOrLive, JigId,
    },
};
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ResourceObject {
    content: serde_json::Value,
}

pub async fn create(
    pool: &PgPool,
    jig_id: JigId,
    display_name: String,
    resource_type_id: Uuid,
    resource_content: ResourceContent,
) -> anyhow::Result<AdditionalResourceId> {
    // Checks if Audio and Image IDs exists
    let resource: serde_json::Value = check_content(pool, resource_content).await?;

    sqlx::query!(
        r#"
insert into jig_data_additional_resource (jig_data_id, resource_type_id, resource_content, display_name)
values ((select draft_id from jig where id = $1), $2, $3, $4)
returning id as "id!: AdditionalResourceId"
        "#,
        jig_id.0,
        resource_type_id,
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
    jig_id: JigId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
) -> anyhow::Result<(String, Uuid, ResourceContent)> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, jig_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get jig_data IDs"))?;

    let jig_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    let res = sqlx::query!(
        r#"
select display_name         as "display_name!",
       resource_type_id     as "resource_type_id!",
       resource_content    as "resource_content!"
from jig_data_additional_resource "jdar"
where jig_data_id = $1
  and jdar.id = $2
        "#,
        jig_data_id,
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
    jig_id: JigId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
    display_name: Option<String>,
    resource_type_id: Option<Uuid>,
    resource_content: Option<ResourceContent>,
) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, jig_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get jig_data IDs"))?;

    let jig_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    if let Some(display_name) = display_name {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data_additional_resource
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
update jig_data_additional_resource
set resource_type_id = coalesce($2, resource_type_id)
where id = $1 and $2 is distinct from resource_type_id
            "#,
            id.0,
            resource_type_id
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(resource_content) = resource_content {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data_additional_resource
set resource_content = $3
where jig_data_id = $1 and id = $2
            "#,
            jig_data_id,
            id.0,
            json!(resource_content)
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, jig_id: JigId, id: AdditionalResourceId) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, jig_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get jig_data IDs"))?;

    sqlx::query!(
        //language=SQL
        r#"
delete
from jig_data_additional_resource
where jig_data_id = $1
   or jig_data_id = $2
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
    };

    Ok(resource)
}
