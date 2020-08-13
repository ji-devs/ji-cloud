use core::settings::req_env;
use rusoto_core::{
    credential::{EnvironmentProvider, ProvideAwsCredentials},
    Region,
};
use rusoto_s3::{
    util::{PreSignedRequest as _, PreSignedRequestOption},
    GetObjectRequest, PutObjectRequest,
};
use shared::domain::image::ImageId;

#[derive(Clone)]
pub struct S3Client {
    creds_provider: EnvironmentProvider,
    region: Region,
    bucket: String,
}

impl S3Client {
    // todo: move it over to core
    pub fn from_env() -> anyhow::Result<Self> {
        let endpoint = req_env("S3_ENDPOINT")?;
        let bucket = req_env("S3_BUCKET")?;

        let region = Region::Custom {
            name: "auto".to_owned(),
            endpoint,
        };

        Ok(Self {
            region,
            creds_provider: EnvironmentProvider::with_prefix("S3"),
            bucket,
        })
    }

    pub async fn presigned_image_get_url(&self, image: ImageId) -> anyhow::Result<String> {
        let url = GetObjectRequest {
            bucket: self.bucket.clone(),
            key: format!("image/{}", image.0.to_hyphenated().to_string()),
            ..GetObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds_provider.credentials().await?,
            &PreSignedRequestOption::default(),
        );

        Ok(url)
    }

    pub async fn presigned_image_put_url(&self, image: ImageId) -> anyhow::Result<String> {
        let url = PutObjectRequest {
            bucket: self.bucket.clone(),
            key: format!("image/{}", image.0.to_hyphenated().to_string()),
            ..PutObjectRequest::default()
        }
        .get_presigned_url(
            &self.region,
            &self.creds_provider.credentials().await?,
            &PreSignedRequestOption::default(),
        );

        Ok(url)
    }
}
