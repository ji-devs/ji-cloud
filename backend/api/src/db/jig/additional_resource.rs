use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::domain::jig::{
    additional_resource::ResourceContent, AdditionalResourceId, DraftOrLive, JigId,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[serde(rename_all = "camelCase")]
pub struct ResourceObject {
    #[serde(alias = "kind")]
    r_kind: &'static str,

    #[serde(flatten)]
    r_content: serde_json::Value,
}

pub async fn create(
    pool: &PgPool,
    jig_id: JigId,
    resource_id: Uuid,
    resource_content: ResourceContent,
) -> anyhow::Result<AdditionalResourceId> {
    let resource: ResourceObject = match resource_content {
        ResourceContent::Image(data) => ResourceObject {
            r_kind: "Image",
            r_content: serde_json::to_value(data)?,
        },
        ResourceContent::Audio(data) => ResourceObject {
            r_kind: "Audio",
            r_content: serde_json::to_value(data)?,
        },
        ResourceContent::Link(data) => ResourceObject {
            r_kind: "Link",
            r_content: serde_json::to_value(data)?,
        },
    };

    sqlx::query!(
        r#"
insert into jig_data_additional_resource (jig_data_id, resource_id, resource_content)
values ((select draft_id from jig where id = $1), $2, $3)
returning id as "id!: AdditionalResourceId"
        "#,
        jig_id.0,
        resource_id,
        json!(resource),
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
) -> anyhow::Result<(String, ResourceContent)> {
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
select display_name     as "display_name!",
       resource_content    as "resource_content!"
from jig_data_additional_resource "jdar"
left join additional_resource "ar" on ar.id = jdar.resource_id
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

    Ok((res.display_name, content))
}

pub async fn update(
    pool: &PgPool,
    jig_id: JigId,
    draft_or_live: DraftOrLive,
    id: AdditionalResourceId,
    resource_id: Option<Uuid>,
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

    if let Some(resource_id) = resource_id {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data_additional_resource
set resource_id= $3
where jig_data_id = $1 and id = $2
            "#,
            jig_data_id,
            id.0,
            resource_id
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(resource_id) = resource_id {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data_additional_resource
set resource_id = $3
where jig_data_id = $1 and id = $2
            "#,
            jig_data_id,
            id.0,
            resource_id
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
