use rusoto_core::{
    credential::{EnvironmentProvider, ProvideAwsCredentials},
    HttpClient, Region,
};
use rusoto_s3::{
    util::{PreSignedRequest as _, PreSignedRequestOption},
    DeleteObjectRequest, GetObjectRequest, PutObjectRequest, S3,
};
use shared::domain::image::ImageId;
use url::Url;

#[derive(Clone)]
pub struct S3Client {
    creds_provider: EnvironmentProvider,
    region: Region,
    bucket: String,
    client: Option<rusoto_s3::S3Client>,
}

fn image_id_to_key(ImageId(id): ImageId) -> String {
    format!("image/{}", id.to_hyphenated())
}

impl S3Client {
    // todo: move it over to core
    pub fn new(endpoint: String, bucket: String, use_client: bool) -> anyhow::Result<Self> {
        let region = Region::Custom {
            name: "auto".to_owned(),
            endpoint,
        };

        let credentials = EnvironmentProvider::with_prefix("S3");

        let client = if use_client {
            Some(rusoto_s3::S3Client::new_with(
                HttpClient::new()?,
                credentials.clone(),
                region.clone(),
            ))
        } else {
            None
        };

        Ok(Self {
            region,
            creds_provider: credentials,
            bucket,
            client,
        })
    }

    pub async fn presigned_image_get_url(&self, image: ImageId) -> anyhow::Result<Url> {
        let url = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: image_id_to_key(image),
            ..GetObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds_provider.credentials().await?,
            &PreSignedRequestOption::default(),
        );

        Ok(url.parse()?)
    }

    pub async fn presigned_image_put_url(&self, image: ImageId) -> anyhow::Result<Url> {
        let url = PutObjectRequest {
            bucket: self.bucket.clone(),
            key: image_id_to_key(image),
            ..PutObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds_provider.credentials().await?,
            &PreSignedRequestOption::default(),
        );

        Ok(url.parse()?)
    }

    // note: does nothing if image doesn't exist, or if the client is disabled.
    pub async fn delete_image(&self, image: ImageId) -> anyhow::Result<()> {
        if let Some(client) = self.client.as_ref() {
            client
                .delete_object(DeleteObjectRequest {
                    key: image_id_to_key(image),
                    bucket: self.bucket.clone(),
                    ..DeleteObjectRequest::default()
                })
                .await?;
        }

        Ok(())
    }
}
