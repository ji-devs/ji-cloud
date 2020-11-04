use shared::domain::jig::{module::Module, ModuleId, ModuleKind};
use sqlx::PgPool;

pub async fn create(
    pool: &PgPool,
    kind: Option<ModuleKind>,
    body: Option<&serde_json::Value>,
) -> anyhow::Result<ModuleId> {
    sqlx::query!(
        r#"insert into module (kind, contents) values ($1, $2) returning id as "id: ModuleId""#,
        kind.map(|it| it as i16),
        body
    )
    .fetch_one(pool)
    .await
    .map(|it| it.id)
    .map_err(Into::into)
}

pub async fn update(
    pool: &PgPool,
    id: ModuleId,
    kind: Option<ModuleKind>,
    body: Option<&serde_json::Value>,
) -> anyhow::Result<bool> {
    sqlx::query!(
        r#"
update module
set contents = coalesce($2, contents),
    kind = coalesce($3, kind)
where id = $1 and (
    ($2::jsonb is not null and $2 is distinct from contents) or
    ($3::int2 is not null and $3 is distinct from kind)
)
returning true as "exists!"
"#,
        id.0,
        body,
        kind.map(|it| it as i16),
    )
    .fetch_optional(pool)
    .await
    .map(|it| it.map_or(false, |it| it.exists))
    .map_err(Into::into)
}

pub async fn get(pool: &PgPool, id: ModuleId) -> anyhow::Result<Option<Module>> {
    sqlx::query_as!(
        Module,
        r#"select id as "id: ModuleId", contents as "body", kind as "kind: ModuleKind" from module where id = $1"#,
        id.0
    )
    .fetch_optional(pool)
    .await
    .map_err(Into::into)
}

pub async fn delete(pool: &PgPool, id: ModuleId) -> anyhow::Result<()> {
    sqlx::query!("delete from module where id = $1", id.0)
        .execute(pool)
        .await
        .map(drop)
        .map_err(Into::into)
}
