use anyhow::Context;
use aws_credential_types::Credentials;
use aws_sdk_s3::{config::Region, primitives::ByteStream, Client as S3Client};
use ji_core::settings::S3Settings;
use shared::media::{self, media_key, FileKind, MediaLibrary, PngImageFile};
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    media_bucket: String,
    client: S3Client,
}

impl Client {
    pub fn new(s3_settings: S3Settings) -> anyhow::Result<Self> {
        let S3Settings {
            endpoint,
            media_bucket,
            access_key_id,
            secret_access_key,
            ..
        } = s3_settings;

        let credentials = Credentials::new(access_key_id, secret_access_key, None, None, "static");

        let config = aws_sdk_s3::config::Builder::new()
            .behavior_version_latest()
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        let client = S3Client::from_conf(config);

        Ok(Self {
            media_bucket,
            client,
        })
    }

    #[instrument(skip(self, library, original, resized, thumbnail))]
    pub async fn upload_png_images(
        &self,
        library: MediaLibrary,
        image: Uuid,
        original: Vec<u8>,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        self.upload_media(
            original,
            library,
            image,
            FileKind::ImagePng(PngImageFile::Original),
        )
        .await?;
        self.upload_media(
            resized,
            library,
            image,
            FileKind::ImagePng(PngImageFile::Resized),
        )
        .await?;
        self.upload_media(
            thumbnail,
            library,
            image,
            FileKind::ImagePng(PngImageFile::Thumbnail),
        )
        .await?;

        Ok(())
    }

    pub async fn delete_media(&self, library: MediaLibrary, file: FileKind, id: Uuid) {
        let key = media_key(library, id, file);
        if let Err(err) = self.try_delete(key.clone()).await {
            log::warn!("failed to delete {} from s3: {}", key, err);
        }
    }

    // note: does nothing if object doesn't exist.
    async fn try_delete(&self, key: String) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.media_bucket)
            .key(key)
            .send()
            .await
            .context("failed to delete object from s3")?;

        Ok(())
    }

    async fn upload_media_to_bucket(
        &self,
        data: Vec<u8>,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
        bucket: String,
    ) -> anyhow::Result<()> {
        let content_length = data.len() as i64;

        self.client
            .put_object()
            .bucket(bucket)
            .key(media::media_key(library, id, file_kind))
            .content_length(content_length)
            .content_type(file_kind.content_type())
            .body(ByteStream::from(data))
            .send()
            .await
            .context("failed to upload object to s3")?;

        Ok(())
    }

    pub async fn upload_media(
        &self,
        data: Vec<u8>,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<()> {
        self.upload_media_to_bucket(data, library, id, file_kind, self.media_bucket.clone())
            .await
    }
}
