pub(crate) mod animation;
pub(crate) mod audio;
pub(crate) mod category;
pub(crate) mod circle;
pub(crate) mod course;
pub(crate) mod image;
pub(crate) mod jig;
pub(crate) mod locale;
pub(crate) mod media;
pub(crate) mod meta;
pub(crate) mod pdf;
pub(crate) mod resource;
pub(crate) mod session;
pub(crate) mod user;

use core::config::DB_POOL_CONNECTIONS;
use shared::domain::{
    category::CategoryId,
    jig::JigId,
    meta::{
        AffiliationId, AgeRangeId, AnimationStyleId, ImageStyleId, ImageTagIndex, ResourceTypeId,
        SubjectId,
    },
};
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    PgConnection,
};
use std::fmt::Write as _;
use tracing::{instrument, Instrument};
use uuid::Uuid;

/// Mirrors the database schema for uploads for each media kind.
///
/// ## Tuple fields:
/// * `0`: media table
/// * `1`: upload status table
/// * `2`: column name in status table, usually {media}_id
pub const UPLOADS_DB_SCHEMA: &[(&str, &str, &str)] = &[
    ("image_metadata", "image_upload", "image_id"),
    ("user_image_library", "user_image_upload", "image_id"),
    ("user_audio_library", "user_audio_upload", "audio_id"),
    (
        "animation_metadata",
        "global_animation_upload",
        "animation_id",
    ),
    ("user_pdf_library", "user_pdf_upload", "pdf_id"),
];

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .connect_with(connect_options)
        .await?;

    log::info!("Running migrations, if there are any");
    sqlx::migrate!("./migrations").run(&pool).await?;
    log::info!("Migrations complete");

    Ok(pool)
}

pub async fn get_test_pool(
    connect_options: PgConnectOptions,
    pool: PgPool,
) -> anyhow::Result<PgPool> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(DB_POOL_CONNECTIONS)
    //     .connect_with(connect_options)
    //     .await?;

    log::info!("Running migrations, if there are any");
    sqlx::migrate!("./migrations").run(&pool).await?;
    log::info!("Migrations complete");

    Ok(pool)
}

trait Metadata: Into<Uuid> + Copy {
    const TABLE: &'static str;
}

impl Metadata for JigId {
    const TABLE: &'static str = "jig";
}

impl Metadata for AffiliationId {
    const TABLE: &'static str = "affiliation";
}

impl Metadata for ResourceTypeId {
    const TABLE: &'static str = "resource_type";
}

impl Metadata for ImageStyleId {
    const TABLE: &'static str = "style";
}

impl Metadata for AnimationStyleId {
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

#[instrument(skip(conn, meta))]
async fn recycle_metadata<'a, T: Metadata>(
    conn: &mut PgConnection,
    table: &str,
    id: Uuid,
    meta: &[T],
) -> sqlx::Result<()> {
    // [Ty] Necessary because for reasons unknown to me, I cannot pass T::TABLE directly into the
    // the info_span! macro as an argument.
    let table_name_suffix = T::TABLE;

    sqlx::query(&format!(
        "delete from {0}_{1} where {0}_id = $1",
        table, table_name_suffix
    ))
    .bind(id)
    .execute(&mut *conn)
    .instrument(tracing::info_span!(
        "delete {}_{}",
        table,
        table_name_suffix
    ))
    .await?;

    for meta in meta.chunks(i16::MAX as usize - 1) {
        let query = generate_metadata_insert(table, T::TABLE, meta.len());

        let mut query = sqlx::query(&query).bind(id);

        for meta in meta {
            let uuid: Uuid = meta.clone().into();
            query = query.bind(uuid);
        }

        query
            .execute(&mut *conn)
            .instrument(tracing::info_span!("insert metadata"))
            .await?;
    }

    Ok(())
}

fn generate_metadata_insert(base_table: &str, meta_kind: &str, binds: usize) -> String {
    debug_assert_ne!(binds, 0);
    debug_assert_ne!(binds, i16::MAX as usize);

    tracing::debug!("Table {}_{}", base_table, meta_kind);

    let mut s = format!(
        "insert into {0}_{1} ({0}_id, {1}_id) values($1, $2)",
        base_table, meta_kind
    );

    for i in 1..binds {
        write!(s, ", ($1, ${})", i + 2).expect("write to String shouldn't fail");
    }

    s
}

trait TagIndex: Into<i16> + Copy {}

impl TagIndex for ImageTagIndex {}

async fn recycle_tags<T: TagIndex>(
    conn: &mut PgConnection,
    table: &str,
    id: Uuid,
    tag_index: &[T],
) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "delete from {0}_tag_join where {0}_id = $1",
        table,
    ))
    .bind(id)
    .execute(&mut *conn)
    .await?;

    for chunk in tag_index.chunks(i16::MAX as usize - 1) {
        let query = generate_tag_insert(table, chunk.len());

        let mut query = sqlx::query(&query).bind(id);

        for tag in chunk {
            let tag: i16 = tag.clone().into();
            query = query.bind(tag);
        }

        query.execute(&mut *conn).await?;
    }

    Ok(())
}

fn generate_tag_insert(base_table: &str, binds: usize) -> String {
    debug_assert_ne!(binds, 0);
    debug_assert_ne!(binds, i16::MAX as usize);

    let mut s = format!(
        "insert into {0}_tag_join ({0}_id, tag_index) values($1, $2)",
        base_table,
    );

    for i in 1..binds {
        write!(s, ", ($1, ${})", i + 2).expect("write to String shouldn't fail");
    }

    s
}

pub(crate) const fn nul_if_empty<T>(arr: &[T]) -> Option<&[T]> {
    if arr.is_empty() {
        None
    } else {
        Some(arr)
    }
}
