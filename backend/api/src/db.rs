pub(crate) mod audio;
pub(crate) mod category;
pub(crate) mod image;
pub(crate) mod jig;
pub(crate) mod meta;
pub(crate) mod module;
pub(crate) mod user;

use config::DB_POOL_CONNECTIONS;
use shared::domain::{
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ContentTypeId, StyleId, SubjectId},
};
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    PgConnection,
};
use std::fmt::Write as _;
use uuid::Uuid;

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

trait Metadata: Into<Uuid> + Copy {
    const TABLE: &'static str;
}

impl Metadata for AffiliationId {
    const TABLE: &'static str = "affiliation";
}

impl Metadata for StyleId {
    const TABLE: &'static str = "style";
}

impl Metadata for AgeRangeId {
    const TABLE: &'static str = "age_range";
}

impl Metadata for CategoryId {
    const TABLE: &'static str = "category";
}

impl Metadata for SubjectId {
    const TABLE: &'static str = "subject";
}

impl Metadata for ContentTypeId {
    const TABLE: &'static str = "content_type";
}

async fn recycle_metadata<'a, T: Metadata>(
    conn: &mut PgConnection,
    table: &str,
    id: Uuid,
    meta: &[T],
) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "delete from {0}_{1} where {0}_id = $1",
        table,
        T::TABLE
    ))
    .bind(id)
    .execute(&mut *conn)
    .await?;

    for meta in meta.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert(table, T::TABLE, meta.len());
        let mut query = sqlx::query(&query).bind(id);

        for meta in meta {
            let uuid: Uuid = meta.clone().into();
            query = query.bind(uuid);
        }

        query.execute(&mut *conn).await?;
    }

    Ok(())
}

fn generate_metadata_insert(base_table: &str, meta_kind: &str, binds: usize) -> String {
    debug_assert_ne!(binds, 0);
    debug_assert_ne!(binds, i16::MAX as usize);

    let mut s = format!(
        "insert into {0}_{1} ({0}_id, {1}_id) values($1, $2)",
        base_table, meta_kind
    );

    for i in 1..binds {
        write!(s, ", ($1, ${})", i + 2).expect("write to String shouldn't fail");
    }

    s
}

pub(crate) fn nul_if_empty<T>(arr: &[T]) -> Option<&[T]> {
    if arr.is_empty() {
        None
    } else {
        Some(arr)
    }
}
