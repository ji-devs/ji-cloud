//! Google Cloud Storage

use crate::error;
use anyhow::Context;
use core::settings::GoogleCloudStorageSettings;
use http::StatusCode;
use reqwest::{self, header};
use shared::media::{self, FileKind, MediaLibrary, PngImageFile};
use uuid::Uuid;

pub struct Client {
    oauth2_token: String,
    media_bucket: String,
    processing_bucket: String,
}

impl Client {
    pub fn new(settings: GoogleCloudStorageSettings) -> anyhow::Result<Self> {
        let GoogleCloudStorageSettings {
            oauth2_token,
            media_bucket,
            processing_bucket,
        } = settings;

        Ok(Self {
            oauth2_token,
            media_bucket,
            processing_bucket,
        })
    }

    pub async fn get_url_for_resumable_upload(
        &self,
        bucket: &str,
        upload_content_length: usize,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> Result<String, error::Storage> {
        let key = media::media_key(library, id, file_kind);

        let resp: reqwest::Response = reqwest::Client::new()
            .post(&format!(
                "https://storage.googleapis.com/upload/storage/v1/b/{}/o",
                bucket
            ))
            .query(&[("uploadType", "resumable"), ("name", &key)])
            .header("X-Upload-Content-Length", upload_content_length.to_string())
            .header("X-Upload-Content-Type", file_kind.content_type().to_owned())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.oauth2_token.to_owned()),
            )
            .header(header::CONTENT_LENGTH, "0")
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let session_uri = resp
                    .headers()
                    .get("Location")
                    .expect(
                        "Should give valid session URI, as long as google storage API is stable",
                    )
                    .to_str()?
                    .to_owned();

                Ok(session_uri)
            }
            StatusCode::UNAUTHORIZED => Err(error::Storage::InvalidGrant),
            _ => {
                let err = resp
                    .json::<UploadUrlErrorResponse>()
                    .await
                    .with_context(|| {
                        anyhow::anyhow!(
                            "Failed to parse resumable upload URL from {}",
                            stringify!(UploadUrlErrorResponse)
                        )
                    })?;

                Err(err.into())
            }
        }
    }

    // https://cloud.google.com/storage/docs/performing-resumable-uploads#initiate-session
    pub async fn get_url_for_resumable_upload_for_processing(
        &self,
        upload_content_length: usize,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> Result<String, error::Storage> {
        self.get_url_for_resumable_upload(
            &self.processing_bucket,
            upload_content_length,
            library,
            id,
            file_kind,
        )
        .await
    }

    pub async fn get_url_for_resumable_upload_for_media(
        &self,
        upload_content_length: usize,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
    ) -> Result<String, error::Storage> {
        self.get_url_for_resumable_upload(
            &self.processing_bucket,
            upload_content_length,
            library,
            id,
            file_kind,
        )
        .await
    }

    pub fn file_size_limit(&self, file_kind: &FileKind) -> Option<usize> {
        match file_kind {
            FileKind::AnimationGif => Some(config::ANIMATION_BODY_SIZE_LIMIT),
            FileKind::ImagePng(PngImageFile::Original) => Some(config::IMAGE_BODY_SIZE_LIMIT),
            _ => unimplemented!("File type size limit undefined!"),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum UploadUrlErrorResponse {
    Unknown(std::collections::HashMap<String, String>),
}
