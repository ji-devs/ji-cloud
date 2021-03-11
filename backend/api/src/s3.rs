use anyhow::Context;
use core::settings::S3Settings;
use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    HttpClient, Region, RusotoError,
};
use rusoto_s3::{DeleteObjectRequest, GetObjectError, GetObjectRequest, PutObjectRequest, S3};
use shared::media::{self, media_key, FileKind, MediaLibrary, PngImageFile};
use tokio::io::AsyncReadExt;
use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    bucket: String,
    client: rusoto_s3::S3Client,
}

impl Client {
    pub fn new(s3_settings: S3Settings) -> anyhow::Result<Self> {
        let S3Settings {
            endpoint,
            bucket,
            access_key_id,
            secret_access_key,
        } = s3_settings;

        let region = Region::Custom {
            name: "auto".to_owned(),
            endpoint,
        };

        let credentials_provider = StaticProvider::from(AwsCredentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
        ));

        let client =
            rusoto_s3::S3Client::new_with(HttpClient::new()?, credentials_provider, region.clone());

        Ok(Self { bucket, client })
    }

    pub async fn upload_png_images_resized_thumb(
        &self,
        library: MediaLibrary,
        image: Uuid,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        let upload = |data, file| self.upload_media(data, library, image, FileKind::ImagePng(file));

        let resized = upload(resized, PngImageFile::Resized);
        let thumbnail = upload(thumbnail, PngImageFile::Thumbnail);

        futures::future::try_join(resized, thumbnail).await?;

        Ok(())
    }

    pub async fn upload_png_images(
        &self,
        library: MediaLibrary,
        image: Uuid,
        original: Vec<u8>,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        let upload = |data, file| self.upload_media(data, library, image, FileKind::ImagePng(file));

        let original = upload(original, PngImageFile::Original);
        let resized = upload(resized, PngImageFile::Resized);
        let thumbnail = upload(thumbnail, PngImageFile::Thumbnail);

        futures::future::try_join3(original, resized, thumbnail).await?;

        Ok(())
    }

    pub async fn delete_media(&self, library: MediaLibrary, file: FileKind, id: Uuid) {
        let key = media_key(library, id, file);
        if let Err(err) = self.try_delete(key.clone()).await {
            log::warn!("failed to delete {} from s3: {}", key, err);

            sentry::with_scope(
                |scope| scope.set_level(Some(sentry::Level::Warning)),
                || {
                    sentry::add_breadcrumb(sentry::Breadcrumb {
                        ty: "info".to_owned(),
                        data: {
                            let mut map = sentry::protocol::Map::new();
                            map.insert("key".to_owned(), key.clone().into());
                            map
                        },
                        ..Default::default()
                    });

                    sentry::integrations::anyhow::capture_anyhow(&err);
                },
            );
        }
    }

    // note: does nothing if object doesn't exist.
    async fn try_delete(&self, key: String) -> anyhow::Result<()> {
        self.client
            .delete_object(DeleteObjectRequest {
                key,
                bucket: self.bucket.clone(),
                ..DeleteObjectRequest::default()
            })
            .await
            .context("failed to delete object from s3")?;

        Ok(())
    }

    pub async fn upload_media(
        &self,
        data: Vec<u8>,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<()> {
        self.client
            .put_object(PutObjectRequest {
                bucket: self.bucket.clone(),
                key: media::media_key(library, id, file_kind),
                content_type: Some(file_kind.content_type().to_owned()),
                body: Some(data.into()),
                ..PutObjectRequest::default()
            })
            .await?;
        Ok(())
    }

    pub async fn download_media_file(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let resp = self
            .client
            .get_object(GetObjectRequest {
                bucket: self.bucket.clone(),
                key: media::media_key(library, id, file_kind),
                ..GetObjectRequest::default()
            })
            .await;

        let resp = match resp {
            Ok(resp) => resp,
            Err(RusotoError::Service(GetObjectError::NoSuchKey(_))) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let mut body = vec![];

        resp.body
            .ok_or_else(|| anyhow::anyhow!("missing response"))?
            .into_async_read()
            .read_to_end(&mut body)
            .await?;

        Ok(Some(body))
    }
}
