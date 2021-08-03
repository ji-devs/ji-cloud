//! Google Cloud Storage
use std::ops::Deref;

use crate::error;
use crate::extractor::RequestOrigin;
use anyhow::Context;
use http::StatusCode;
use reqwest::{self, header};
use uuid::Uuid;

use core::{
    config::{ANIMATION_BODY_SIZE_LIMIT, IMAGE_BODY_SIZE_LIMIT},
    settings::GoogleCloudStorageSettings,
};
use shared::{
    config::CORS_ORIGINS,
    media::{self, FileKind, MediaLibrary, PngImageFile},
};

pub struct Client {
    oauth2_token: String,
    #[allow(dead_code)]
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
        origin: RequestOrigin,
    ) -> Result<String, error::Storage> {
        let key = media::media_key(library, id, file_kind);

        let req = reqwest::Client::new()
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
            .header(header::CONTENT_LENGTH, "0");

        let req = match origin.origin {
            Some(origin) if CORS_ORIGINS.contains(&origin.deref()) => {
                req.header(header::ORIGIN, origin)
            }
            _ => req,
        };

        let resp: reqwest::Response = req.send().await?;
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
            // StatusCode::UNAUTHORIZED => Err(error::Storage::InvalidGrant),
            _ => {
                // FIXME
                log::warn!(
                    "{:?}",
                    resp.json::<serde_json::Value>()
                        .await
                        .expect("debug error decode!")
                );

                // let err = resp
                //     .json::<UploadUrlErrorResponse>()
                //     .await
                //     .with_context(|| {
                //         anyhow::anyhow!(
                //             "Failed to parse resumable upload URL from {}",
                //             stringify!(UploadUrlErrorResponse)
                //         )
                //     })?;

                let err = anyhow::anyhow!("see logs...");

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
        origin: RequestOrigin,
    ) -> Result<String, error::Storage> {
        self.get_url_for_resumable_upload(
            &self.processing_bucket,
            upload_content_length,
            library,
            id,
            file_kind,
            origin,
        )
        .await
    }

    pub async fn get_url_for_resumable_upload_for_media(
        &self,
        upload_content_length: usize,
        library: MediaLibrary,
        id: Uuid,
        file_kind: FileKind,
        origin: RequestOrigin,
    ) -> Result<String, error::Storage> {
        self.get_url_for_resumable_upload(
            &self.processing_bucket,
            upload_content_length,
            library,
            id,
            file_kind,
            origin,
        )
        .await
    }

    pub fn file_size_limit(&self, file_kind: &FileKind) -> Option<usize> {
        match file_kind {
            FileKind::AnimationGif => Some(ANIMATION_BODY_SIZE_LIMIT),
            FileKind::ImagePng(PngImageFile::Original) => Some(IMAGE_BODY_SIZE_LIMIT),
            _ => unimplemented!("File type size limit undefined!"),
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum UploadUrlErrorResponse {
    Unknown(std::collections::HashMap<String, String>),
}
