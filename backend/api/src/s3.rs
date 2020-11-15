use core::settings::S3Settings;
use rusoto_core::{
    credential::{AwsCredentials, StaticProvider},
    HttpClient, Region,
};
use rusoto_s3::{
    util::{PreSignedRequest as _, PreSignedRequestOption},
    DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3,
};
use shared::domain::image::ImageId;
use url::Url;

#[derive(Debug, Copy, Clone)]
pub enum S3ImageKind {
    Original,
    Resized,
    Thumbnail,
}

impl S3ImageKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Resized => "resized",
            Self::Thumbnail => "thumbnail",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum S3LibraryKind {
    Global,
    User,
    Web,
}

impl S3LibraryKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Global => "image",
            Self::User => "image-user",
            Self::Web => "image-web",
        }
    }
}

#[derive(Clone)]
pub struct S3Client {
    creds: AwsCredentials,
    region: Region,
    bucket: String,
    client: Option<rusoto_s3::S3Client>,
}

fn image_id_to_key(library: S3LibraryKind, kind: S3ImageKind, ImageId(id): ImageId) -> String {
    format!(
        "{}/{}/{}",
        library.as_str(),
        kind.as_str(),
        id.to_hyphenated()
    )
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
        library: S3LibraryKind,
        image: ImageId,
        original: Vec<u8>,
        resized: Vec<u8>,
        thumbnail: Vec<u8>,
    ) -> anyhow::Result<()> {
        let client = match &self.client {
            Some(client) => client,
            None => return Ok(()),
        };

        let id_to_key = |kind| image_id_to_key(library, kind, image);

        let original = client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            key: id_to_key(S3ImageKind::Original),
            body: Some(original.into()),
            ..PutObjectRequest::default()
        });

        let resized = client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            key: id_to_key(S3ImageKind::Resized),
            body: Some(resized.into()),
            ..PutObjectRequest::default()
        });

        let thumbnail = client.put_object(PutObjectRequest {
            bucket: self.bucket.clone(),
            key: id_to_key(S3ImageKind::Thumbnail),
            body: Some(thumbnail.into()),
            ..PutObjectRequest::default()
        });

        futures::future::try_join3(original, resized, thumbnail).await?;

        Ok(())
    }

    pub fn presigned_image_get_url(
        &self,
        library: S3LibraryKind,
        kind: S3ImageKind,
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

    pub async fn delete_image(&self, library: S3LibraryKind, kind: S3ImageKind, image: ImageId) {
        if let Err(err) = self.try_delete_image(library, kind, image).await {
            log::warn!(
                "failed to delete image with id {} ({}) from s3: {}",
                image.0.to_hyphenated(),
                kind.as_str(),
                err
            );
        }
    }

    // note: does nothing if image doesn't exist, or if the client is disabled.
    pub async fn try_delete_image(
        &self,
        library: S3LibraryKind,
        kind: S3ImageKind,
        image: ImageId,
    ) -> anyhow::Result<()> {
        if let Some(client) = self.client.as_ref() {
            client
                .delete_object(DeleteObjectRequest {
                    key: image_id_to_key(library, kind, image),
                    bucket: self.bucket.clone(),
                    ..DeleteObjectRequest::default()
                })
                .await?;
        }

        Ok(())
    }
}
