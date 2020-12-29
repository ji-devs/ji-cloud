use anyhow::Context;
use core::settings::S3Settings;
use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    HttpClient, Region,
};
use rusoto_s3::{
    util::{PreSignedRequest as _, PreSignedRequestOption},
    DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3,
};
use shared::{
    domain::{audio::AudioId, image::ImageId},
    media::{audio_id_to_key, image_id_to_key, Audio, ImageSize, MediaLibraryKind},
};
use url::Url;

#[derive(Clone)]
pub struct S3Client {
    creds: AwsCredentials,
    region: Region,
    bucket: String,
    client: Option<rusoto_s3::S3Client>,
}

impl S3Client {
    pub fn new(s3_settings: S3Settings) -> anyhow::Result<Self> {
        let S3Settings {
            endpoint,
            bucket,
            access_key_id,
            secret_access_key,
            use_client,
        } = s3_settings;

        let region = Region::Custom {
            name: "auto".to_owned(),
            endpoint,
        };

        let creds = AwsCredentials::new(access_key_id, secret_access_key, None, None);

        let credentials_provider = StaticProvider::from(creds.clone());

        let client = if use_client {
            Some(rusoto_s3::S3Client::new_with(
                HttpClient::new()?,
                credentials_provider,
                region.clone(),
            ))
        } else {
            None
        };

        Ok(Self {
            region,
            creds,
            bucket,
            client,
        })
    }

    pub async fn upload_images(
        &self,
        library: MediaLibraryKind,
        image: ImageId,
        original: Vec<u8>,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        let client = match &self.client {
            Some(client) => client,
            None => return Ok(()),
        };

        let make_req = |kind, body: Vec<u8>| PutObjectRequest {
            bucket: self.bucket.clone(),
            key: image_id_to_key(library, kind, image),
            body: Some(body.into()),
            content_type: Some("image/png".to_owned()),
            ..PutObjectRequest::default()
        };

        let original = client.put_object(make_req(ImageSize::Original, original));
        let resized = client.put_object(make_req(ImageSize::Resized, resized));
        let thumbnail = client.put_object(make_req(ImageSize::Thumbnail, thumbnail));

        futures::future::try_join3(original, resized, thumbnail).await?;

        Ok(())
    }

    pub fn image_presigned_get_url(
        &self,
        library: MediaLibraryKind,
        kind: ImageSize,
        image: ImageId,
    ) -> anyhow::Result<Url> {
        let url = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: image_id_to_key(library, kind, image),
            ..GetObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds,
            &PreSignedRequestOption::default(),
        );

        Ok(url.parse()?)
    }

    async fn delete_media(&self, key: &str) {
        if let Err(err) = self.try_delete(key.to_string()).await {
            log::warn!("failed to delete {} from s3: {}", key, err);

            sentry::with_scope(
                |scope| scope.set_level(Some(sentry::Level::Warning)),
                || {
                    sentry::add_breadcrumb(sentry::Breadcrumb {
                        ty: "info".to_owned(),
                        data: {
                            let mut map = sentry::protocol::Map::new();
                            map.insert("key".to_owned(), key.to_string().into());
                            map
                        },
                        ..Default::default()
                    });

                    sentry::integrations::anyhow::capture_anyhow(&err);
                },
            );
        }
    }

    // note: does nothing if object doesn't exist, or if the client is disabled.
    async fn try_delete(&self, key: String) -> anyhow::Result<()> {
        if let Some(client) = self.client.as_ref() {
            client
                .delete_object(DeleteObjectRequest {
                    key,
                    bucket: self.bucket.clone(),
                    ..DeleteObjectRequest::default()
                })
                .await
                .context("failed to delete object from s3")?;
        }

        Ok(())
    }

    pub async fn delete_image(
        &self,
        library: MediaLibraryKind,
        variant: ImageSize,
        image: ImageId,
    ) {
        self.delete_media(&image_id_to_key(library, variant, image))
            .await
    }

    pub async fn upload_audio(
        &self,
        library: MediaLibraryKind,
        audio: AudioId,
        original: Vec<u8>,
    ) -> anyhow::Result<()> {
        let client = match &self.client {
            Some(client) => client,
            None => return Ok(()),
        };

        let id_to_key = |kind| audio_id_to_key(library, kind, audio);

        client
            .put_object(PutObjectRequest {
                bucket: self.bucket.clone(),
                key: id_to_key(Audio::Original),
                body: Some(original.into()),
                content_type: Some("audio/mp3".to_owned()),
                ..PutObjectRequest::default()
            })
            .await?;

        Ok(())
    }

    pub fn audio_presigned_get_url(
        &self,
        library: MediaLibraryKind,
        kind: Audio,
        audio: AudioId,
    ) -> anyhow::Result<Url> {
        let url = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: audio_id_to_key(library, kind, audio),
            ..GetObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds,
            &PreSignedRequestOption::default(),
        );

        Ok(url.parse()?)
    }

    pub async fn delete_audio(&self, library: MediaLibraryKind, variant: Audio, audio: AudioId) {
        self.delete_media(&audio_id_to_key(library, variant, audio))
            .await
    }
}
