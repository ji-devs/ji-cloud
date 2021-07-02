use anyhow::Context;
use core::settings::S3Settings;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    HttpClient, Region, RusotoError,
};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{
    CopyObjectRequest, DeleteObjectRequest, GetObjectError, GetObjectRequest, PutObjectRequest, S3,
};
use shared::media::{self, media_key, FileKind, MediaLibrary, PngImageFile};
use tokio::io::AsyncReadExt;
use url::Url;
use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    media_bucket: String,
    processing_bucket: String,
    client: rusoto_s3::S3Client,
    credentials_provider: StaticProvider, // get rid of,
    region: Region,
}

impl Client {
    pub fn new(s3_settings: S3Settings) -> anyhow::Result<Self> {
        let S3Settings {
            endpoint,
            media_bucket,
            processing_bucket,
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

        let client = rusoto_s3::S3Client::new_with(
            HttpClient::new()?,
            credentials_provider.clone(),
            region.clone(),
        );

        Ok(Self {
            media_bucket,
            processing_bucket,
            client,
            credentials_provider,
            region,
        })
    }

    pub async fn upload_png_images_copy_original(
        &self,
        library: MediaLibrary,
        image: Uuid,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        futures::future::try_join(
            self.copy_processed_file(library, image, FileKind::ImagePng(PngImageFile::Original)),
            self.upload_png_images_resized_thumb(library, image, resized, thumbnail),
        )
        .await
        .map(drop)
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
                bucket: self.media_bucket.clone(),
                ..DeleteObjectRequest::default()
            })
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
        self.client
            .put_object(PutObjectRequest {
                bucket,
                key: media::media_key(library, id, file_kind),
                content_type: Some(file_kind.content_type().to_owned()),
                body: Some(data.into()),
                ..PutObjectRequest::default()
            })
            .await?;

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

    pub async fn upload_media_for_processing(
        &self,
        data: Vec<u8>,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<()> {
        self.upload_media_to_bucket(data, library, id, file_kind, self.processing_bucket.clone())
            .await
    }

    pub async fn download_media_for_processing(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        self.download_media_file_from_bucket(self.processing_bucket.clone(), library, id, file_kind)
            .await
    }

    pub async fn download_media_file(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        self.download_media_file_from_bucket(self.media_bucket.clone(), library, id, file_kind)
            .await
    }

    pub async fn copy_processed_file(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<()> {
        let key = media::media_key(library, id, file_kind);
        self.client
            .copy_object(CopyObjectRequest {
                bucket: self.media_bucket.clone(),
                content_type: Some(file_kind.content_type().to_owned()),
                copy_source: format!("{}/{}", self.processing_bucket, key),
                key: key.clone(),
                ..CopyObjectRequest::default()
            })
            .await?;

        Ok(())
    }

    pub async fn back_copy_unprocessed_file(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<()> {
        let key = media::media_key(library, id, file_kind);
        self.client
            .copy_object(CopyObjectRequest {
                bucket: self.processing_bucket.clone(),
                content_type: Some(file_kind.content_type().to_owned()),
                copy_source: format!("{}/{}", self.media_bucket, key),
                key: key.clone(),
                ..CopyObjectRequest::default()
            })
            .await?;

        Ok(())
    }

    async fn download_media_file_from_bucket(
        &self,
        bucket: String,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let resp = self
            .client
            .get_object(GetObjectRequest {
                bucket,
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

    pub async fn get_presigned_url_to_upload_media_for_processing(
        &self,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> anyhow::Result<Url> {
        // FIXME: should probably use self.client, but S3Client doesn't support signed URLs.

        let option = PreSignedRequestOption {
            expires_in: std::time::Duration::from_secs(config::MEDIA_UPLOAD_TIMEOUT_SECS),
        };

        let req = PutObjectRequest {
            bucket: self.processing_bucket.clone(),
            key: media::media_key(library, id, file_kind),
            ..PutObjectRequest::default()
        };

        let url = req.get_presigned_url(
            &self.region,
            &self.credentials_provider.credentials().await?,
            &option,
        );

        match Url::parse(&url) {
            Ok(url) => Ok(url),
            Err(e) => Err(e.into()),
        }
    }
}
