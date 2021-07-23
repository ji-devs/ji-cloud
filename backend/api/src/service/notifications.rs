use core::settings::FirebaseCloudMessageSettings;
use reqwest::{self, header, StatusCode};
use serde::Serialize;
use shared::domain::firebase::{FirebaseCloudMessage, MessageTarget};

#[derive(Debug)]
pub struct Client {
    oauth2_token: String,
    project_id: String,
}

impl Client {
    pub fn new(settings: FirebaseCloudMessageSettings) -> anyhow::Result<Self> {
        let FirebaseCloudMessageSettings {
            oauth2_token,
            project_id,
        } = settings;

        Ok(Self {
            oauth2_token,
            project_id,
        })
    }

    pub async fn send_message(&self, message: MessageRequest) -> anyhow::Result<()> {
        let resp = reqwest::Client::new()
            .post(&format!(
                "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                self.project_id
            ))
            .json(&message)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.oauth2_token.to_owned()),
            )
            .send()
            .await?;

        match resp.status() {
            StatusCode::UNAUTHORIZED => (),
            _ => (),
        }

        Ok(())
    }
}

// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send#request-body
#[derive(Serialize)]
pub struct MessageRequest {
    pub validate_only: bool,
    pub message: FirebaseCloudMessage,
}

impl MessageRequest {
    pub fn with_data(target: MessageTarget, data: serde_json::Value) -> Self {
        Self {
            validate_only: false,
            message: FirebaseCloudMessage {
                name: None,
                data,
                target,
            },
        }
    }
}

pub struct RequestBuilder {
    request: MessageRequest, // make this a Result, consumed during build()
}

impl RequestBuilder {
    pub fn named(&mut self, name: String) -> &mut RequestBuilder {
        self.request.message.name = Some(name);
        self
    }

    pub fn target(&mut self, target: MessageTarget) -> &mut RequestBuilder {
        self.request.message.target = target;
        self
    }

    pub fn json_data<T: Serialize>(&mut self, data: serde_json::Value) -> &mut RequestBuilder {
        self.request.message.data = data;
        self
    }

    pub fn build(self) -> MessageRequest {
        self.request
    }
}
