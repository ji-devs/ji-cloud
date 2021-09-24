use shared::domain::jig::{AdditionalResourceId, JigId};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    parent: JigId,
    url: String,
) -> anyhow::Result<AdditionalResourceId> {
    sqlx::query!(
        r#"
insert into jig_additional_resource (jig_id, url)
values ($1, $2)
returning id
        "#,
        parent.0,
        url,
    )
    .fetch_one(pool)
    .await
    .map(|it| AdditionalResourceId(it.id))
    .map_err(Into::into)
}

pub async fn get(
    pool: &PgPool,
    parent: JigId,
    id: AdditionalResourceId,
) -> anyhow::Result<Option<String>> {
    let url = sqlx::query!(
        r#"
select
    url as "url!: String"
from jig_additional_resource
where jig_id = $1 and id = $2
        "#,
        parent.0,
        id.0,
    )
    .fetch_optional(pool)
    .await?
    .map(|it| it.url);

    Ok(url)
}

pub async fn update(
    pool: &PgPool,
    parent_id: JigId,
    id: AdditionalResourceId,
    url: Option<String>,
) -> anyhow::Result<bool> {
    let mut txn = pool.begin().await?;

    sqlx::query!(
        r#"
update jig_additional_resource
set url = coalesce($3, url)
where jig_id = $1 and id = $2
        "#,
        parent_id.0,
        id.0,
        url,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(true)
}

pub async fn delete(
    pool: &PgPool,
    parent_id: JigId,
    id: AdditionalResourceId,
) -> anyhow::Result<()> {
    let mut txn = pool.begin().await?;

    sqlx::query!(
        r#"
delete from jig_additional_resource
where jig_id = $1 and id = $2
        "#,
        parent_id.0,
        id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}
