use ji_core::config::DB_POOL_CONNECTIONS;
use shared::domain::{course::CourseId, jig::JigId, module::ModuleId, playlist::PlaylistId};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

pub async fn get_pool(connect_options: PgConnectOptions) -> anyhow::Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(DB_POOL_CONNECTIONS)
        .connect_with(connect_options)
        .await?)
}

pub struct AssetMetadata {
    pub display_name: String,
    pub description: String,
    pub other_keywords: String,
    pub cover_module_id: Option<ModuleId>,
}

pub async fn get_jig_metadata(db: &PgPool, jig_id: JigId) -> sqlx::Result<Option<AssetMetadata>> {
    let row = sqlx::query!(
        r#"
            select
                jig_data.display_name as "display_name!",
                jig_data.description as "description!",
                jig_data.other_keywords as "other_keywords!",
                (
                    select id
                    from jig_data_module
                    where jig_data_id = jig_data.id
                    order by "index"
                    limit 1
                ) as "cover_module_id?: ModuleId"
            from jig
                inner join jig_data on jig_data.id = coalesce(jig.live_id, jig.draft_id)
            where jig.id = $1
        "#,
        jig_id.0,
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| AssetMetadata {
        display_name: row.display_name,
        description: row.description,
        other_keywords: row.other_keywords,
        cover_module_id: row.cover_module_id,
    }))
}

pub async fn get_playlist_metadata(
    db: &PgPool,
    playlist_id: PlaylistId,
) -> sqlx::Result<Option<AssetMetadata>> {
    let row = sqlx::query!(
        r#"
            select
                playlist_data.display_name as "display_name!",
                playlist_data.description as "description!",
                playlist_data.other_keywords as "other_keywords!",
                (
                    select id
                    from playlist_data_module
                    where playlist_data_id = playlist_data.id
                    order by "index"
                    limit 1
                ) as "cover_module_id?: ModuleId"
            from playlist
                inner join playlist_data on playlist_data.id = coalesce(playlist.live_id, playlist.draft_id)
            where playlist.id = $1
        "#,
        playlist_id.0,
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| AssetMetadata {
        display_name: row.display_name,
        description: row.description,
        other_keywords: row.other_keywords,
        cover_module_id: row.cover_module_id,
    }))
}

pub async fn get_course_metadata(
    db: &PgPool,
    course_id: CourseId,
) -> sqlx::Result<Option<AssetMetadata>> {
    let row = sqlx::query!(
        r#"
            select
                course_data.display_name as "display_name!",
                course_data.description as "description!",
                course_data.other_keywords as "other_keywords!",
                (
                    select id
                    from course_data_module
                    where course_data_id = course_data.id
                    order by "index"
                    limit 1
                ) as "cover_module_id?: ModuleId"
            from course
                inner join course_data on course_data.id = coalesce(course.live_id, course.draft_id)
            where course.id = $1
        "#,
        course_id.0,
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| AssetMetadata {
        display_name: row.display_name,
        description: row.description,
        other_keywords: row.other_keywords,
        cover_module_id: row.cover_module_id,
    }))
}
