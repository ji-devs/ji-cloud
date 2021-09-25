use shared::domain::jig::{AdditionalResourceId, DraftOrLive, JigId};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    jig_id: JigId,
    url: String,
) -> anyhow::Result<AdditionalResourceId> {
    sqlx::query!(
        r#"
insert into jig_data_additional_resource (jig_data_id, url)
values ((select draft_id from jig where id = $1), $2)
returning id as "id!: AdditionalResourceId"
        "#,
        jig_id.0,
        url,
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
) -> anyhow::Result<Option<String>> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = super::get_draft_and_live_ids(&mut txn, jig_id)
        .await
        .ok_or(anyhow::anyhow!("failed to get jig_data IDs"))?;

    let jig_data_id = match draft_or_live {
        DraftOrLive::Draft => draft_id,
        DraftOrLive::Live => live_id,
    };

    let url = sqlx::query!(
        r#"
select url as "url!: String"
from jig_data_additional_resource
where jig_data_id = $1
  and id = $2
        "#,
        jig_data_id,
        id.0,
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.url);

    txn.rollback().await?;

    Ok(url)
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
