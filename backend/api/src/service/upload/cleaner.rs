use anyhow::Context;
use core::config::{EXPIRED_UPLOAD_CLEANUP_PERIOD, UPLOAD_EXPIRY_TIME};
use sqlx::PgPool;
use tokio::{
    task::JoinHandle,
    time::{Duration, Instant},
};

/// Generate query to delete failed old uploads
///
/// # Args
/// * media_table: e.g. 'image_metadata', 'user_image_library'
/// * upload_table: e.g. 'image_upload', 'user_image_upload'
/// * media_id_column: e.g. 'image_id', 'audio_id'
fn generate_query_purge_failed_media_uploads(
    media_table: &str,
    upload_table: &str,
    media_id_column: &str,
) -> String {
    format!(
        r#"with del_ids as (
            select id
            from {0}
                     join {1} on id = {2}
            where created_at < (now() - interval '{3} seconds')
              and processed_at is not distinct from null
        ),
             del_uploads as (delete
                 from {1}
                     where {2} in (select id from del_ids))
        delete
        from {0}
        where id in (select id from del_ids);"#,
        media_table, upload_table, media_id_column, UPLOAD_EXPIRY_TIME
    )
}

#[derive(Debug)]
pub struct MediaUploadDbSchema {
    /// Parent table for the media kind
    pub media_table: &'static str,
    /// Table holding upload status information for the corresponding media kind
    pub upload_table: &'static str,
    /// Name of the foreign key'd column in the upload status table
    pub media_id_column: &'static str,
}

pub struct UploadCleaner {
    db: PgPool,
    /// information about which tables to clean from
    media_schema: Vec<MediaUploadDbSchema>,
}

impl UploadCleaner {
    pub fn new(db: PgPool, media_schema: &'static [(&str, &str, &str)]) -> Self {
        let media_schema = media_schema
            .iter()
            .map(|it| MediaUploadDbSchema {
                media_table: it.0,
                upload_table: it.1,
                media_id_column: it.2,
            })
            .collect();

        Self { db, media_schema }
    }

    #[must_use]
    pub fn spawn(self) -> JoinHandle<()> {
        log::debug!("spawning background expired media upload cleanup task");
        let mut turn_modulus: usize = 0;

        tokio::task::spawn(async move {
            loop {
                let iteration_start = Instant::now();

                let media_type_schema = &self.media_schema[turn_modulus];
                log::debug!("Cleaning table {:?}", media_type_schema);

                turn_modulus = (turn_modulus + 1) % &self.media_schema.len();

                let res = self
                    .clean_up_media(media_type_schema)
                    .await
                    .context(format!(
                        "media upload DB cleanup task failed: {}",
                        media_type_schema.media_table
                    ));

                match res {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{:?}", e);
                        sentry::integrations::anyhow::capture_anyhow(&e);
                    }
                }

                tokio::time::sleep_until(
                    (iteration_start + Duration::from_secs(EXPIRED_UPLOAD_CLEANUP_PERIOD)).into(),
                )
                .await;
            }
        })
    }

    async fn clean_up_media(&self, media_type_schema: &MediaUploadDbSchema) -> anyhow::Result<()> {
        // TODO: should these be pre-generated instead of during runtime?
        let query = generate_query_purge_failed_media_uploads(
            &media_type_schema.media_table,
            &media_type_schema.upload_table,
            &media_type_schema.media_id_column,
        );

        let mut txn = self.db.begin().await?;

        sqlx::query(&query).execute(&mut txn).await?;

        txn.commit().await?;

        Ok(())
    }
}
