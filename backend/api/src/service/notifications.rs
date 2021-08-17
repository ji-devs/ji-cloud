use core::settings::FirebaseSettings;
use reqwest::{self, header, StatusCode};
use serde::{Deserialize, Serialize};
use shared::media::MediaLibrary;
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    project_id: String,
}

/// FIXME use an external crate that actually uses serde <-> REST
/// Current implementation is _very_ limited in scope for updating media upload processing.    
impl Client {
    pub fn new(settings: FirebaseSettings) -> anyhow::Result<Self> {
        let FirebaseSettings { project_id } = settings;

        Ok(Self { project_id })
    }

    pub async fn signal_status(
        &self,
        access_token: &str,
        library: &MediaLibrary,
        id: &Uuid,
        status: ProcessingStatus,
        update_mask: &[&str],
    ) -> anyhow::Result<()> {
        let path_to_document = format!("uploads/media/{}/{}", library.to_str(), id.to_string());

        let update_mask_query: Vec<_> = update_mask
            .into_iter()
            .map(|field| ("updateMask.fieldPaths", *field))
            .collect();

        let document = Document {
            name: None,
            fields: serde_json::to_value(status).ok(),
            create_time: None,
            update_time: None,
        };

        let resp = reqwest::Client::new()
            .patch(&format!(
                "https://firestore.googleapis.com/v1beta1/projects/{}/databases/(default)/documents/{}",
                self.project_id,
                path_to_document,
            ))
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_owned()),
            )
            .query(&update_mask_query)
            .json(&document)
            .send()
            .await
            .map_err(|e| {
                log::warn!("?? {:?}", e);
                anyhow::anyhow!("Unknown error during request to firestore: {:?}", e) }
            )?;

        match resp.status() {
            StatusCode::OK => Ok(()),
            StatusCode::UNAUTHORIZED => {
                Err(anyhow::anyhow!("Unauthorized request to firestore").into())
            }
            StatusCode::FORBIDDEN => Err(anyhow::anyhow!("Forbidden request to firestore").into()),
            _ => {
                let body: serde_json::Value = resp.json().await?;
                Err(anyhow::anyhow!("Unknown status from firestore: {:?}", body).into())
            }
        }
    }

    pub async fn signal_status_processing(
        &self,
        access_token: &str,
        library: &MediaLibrary,
        id: &Uuid,
    ) -> anyhow::Result<()> {
        let status = ProcessingStatus {
            processing: Some(Value::BooleanValue(true)),
            ready: Some(Value::BooleanValue(false)),
        };
        let update_mask = &["processing", "ready"];

        self.signal_status(access_token, library, id, status, update_mask)
            .await?;

        Ok(())
    }

    pub async fn signal_status_ready(
        &self,
        access_token: &str,
        library: &MediaLibrary,
        id: &Uuid,
    ) -> anyhow::Result<()> {
        let status = ProcessingStatus {
            processing: None,
            ready: Some(Value::BooleanValue(true)),
        };
        let update_mask = &["ready"];

        self.signal_status(access_token, library, id, status, update_mask)
            .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProcessingStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    ready: Option<Value>, // bool
    #[serde(skip_serializing_if = "Option::is_none")]
    processing: Option<Value>, // bool
}

// Types for bindings to Firestore

/// https://firebase.google.com/docs/firestore/reference/rest/v1beta1/projects.databases.documents#Document
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<String>,
}

/// Incomplete. See link for full list of data types supported by firestore:
/// https://firebase.google.com/docs/firestore/reference/rest/v1beta1/Value
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Value {
    NullValue, // does this work?
    BooleanValue(bool),
    StringValue(String),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMask {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    field_paths: Vec<String>,
}
