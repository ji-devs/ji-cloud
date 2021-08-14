//! EventArc
//!
//! NOTE: Has nothing to do with Arc<_>. EventArc is the name of the Google Cloud Project service.

use crate::error;
use core::settings::GoogleCloudEventArcSettings;
use serde::{Deserialize, Serialize};
use shared::media::{FileKind, MediaLibrary};
use uuid::Uuid;

#[derive(Debug)]
pub struct Client {
    project_id: String,
    storage_service_name: String,
    media_uploaded_topic: String,
    media_processed_topic: String,
}

impl Client {
    pub fn new(settings: GoogleCloudEventArcSettings) -> anyhow::Result<Self> {
        let GoogleCloudEventArcSettings {
            project_id,
            storage_service_name,
            media_uploaded_topic,
            media_processed_topic,
        } = settings;

        Ok(Self {
            project_id,
            storage_service_name,
            media_uploaded_topic,
            media_processed_topic,
        })
    }

    pub fn project_id(&self) -> &str {
        &self.project_id
    }

    pub fn storage_service_name(&self) -> &str {
        &self.storage_service_name
    }

    // TODO: this
    pub async fn validate_pub_sub_event_source() {
        unimplemented!();
    }
}

// TODO: get rid of the ownership here?
#[derive(Debug)]
pub struct EventSource {
    pub service_name: String,
    pub project_id: String,
}

impl std::str::FromStr for EventSource {
    type Err = error::EventArc;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME: is there a cleaner way to handle this?
        let mut source = s
            .split("/")
            .enumerate()
            .filter(|(n, _)| *n == 2 || *n == 4)
            .map(|(_, e)| e);

        if let (Some(service_name), Some(project_id)) = (source.next(), source.next()) {
            Ok(EventSource {
                service_name: service_name.to_owned(),
                project_id: project_id.to_owned(),
            })
        } else {
            Err(error::EventArc::InvalidEventSource)
        }
    }
}

#[derive(Debug)]
pub struct EventResource {
    pub bucket: String,
    pub library: MediaLibrary,
    pub id: Uuid,
    pub file_kind: FileKind,
}

impl std::str::FromStr for EventResource {
    type Err = error::EventArc;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME: is there a cleaner way to handle this?
        // valid format: "projects/_/buckets/<BUCKET>/objects/media/<LIBRARY>/<ID>/<FILE_KIND>"
        let mut resource = s.split("/");

        // check that the resource name starts with the correct string
        if let Some(prefix) = resource.next() {
            if prefix != "projects" {
                return Err(error::EventArc::InvalidEventSource);
            }
        }

        let _ = resource.next(); // "_/buckets/"
        let _ = resource.next();

        let bucket = resource.next();

        let _ = resource.next(); // "objects/media/"
        let _ = resource.next();

        let library = resource.next();
        let id = resource.next();
        let file_kind = resource.next();

        if let (Some(bucket), Some(library), Some(id), Some(file_kind)) =
            (bucket, library, id, file_kind)
        {
            let bucket = bucket.to_owned();

            let library =
                MediaLibrary::from_str(library).map_err(|_| Self::Err::InvalidEventResource)?;
            let id = Uuid::from_str(id).map_err(|_| Self::Err::InvalidEventResource)?;
            let file_kind =
                FileKind::from_str(file_kind).map_err(|_| Self::Err::InvalidEventResource)?;

            Ok(EventResource {
                bucket,
                library,
                id,
                file_kind,
            })
        } else {
            Err(error::EventArc::InvalidEventResource)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[repr(i16)]
enum EventType {
    StorageCreate = 0,
}

/// Structs to handle the unwieldy data sent as a part of the data field of EventArc audit logs
/// Fields irrelevant to the event handling are skipped
/// See link for JSON spec of request: https://github.com/googleapis/google-cloudevents#cloudevents-in-this-repository
pub mod audit_log {
    use crate::error;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::convert::TryFrom;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Event {
        #[serde(rename = "specversion")]
        pub spec_version: String,
        pub id: String,
        #[serde(rename = "type")]
        pub event_type: String,
        pub source: String,
        #[serde(rename = "datacontenttype")]
        pub data_content_type: String,
        pub time: String,
        pub data_base64: String, // base64 encoded `Data`
    }

    // only handles V1.0 Google Cloud events
    // FIXME: is there an easier way to decode into the CloudEvent type directly? impl From<Event>?
    impl TryFrom<cloudevents::Event> for Event {
        type Error = error::EventArc;

        fn try_from(value: cloudevents::Event) -> Result<Self, Self::Error> {
            let json: serde_json::Value = serde_json::to_value(value)?;
            serde_json::from_value(json).map_err(|_| error::EventArc::InvalidEventResource)
        }
    }

    impl Event {
        pub fn try_decode_event_payload(&self) -> Result<Data, error::EventArc> {
            let event_data_u8 = base64::decode(&self.data_base64)?;
            serde_json::from_slice(&event_data_u8)
                .map_err(|_| error::EventArc::InvalidEventResource)
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        pub proto_payload: ProtoPayload,
        pub insert_id: String,
        pub resource: Resource,
        pub timestamp: DateTime<Utc>,
        pub severity: String,
        pub log_name: String,
        pub receive_timestamp: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ProtoPayload {
        #[serde(skip)]
        pub status: Option<String>,
        #[serde(skip)]
        pub authentication_info: String,
        #[serde(skip)]
        pub request_metadata: String,
        pub service_name: String,
        pub method_name: String,
        #[serde(skip)]
        pub authorization_info: String,
        pub resource_name: String,
        #[serde(skip)]
        pub service_data: String,
        #[serde(skip)]
        pub resource_location: String,
    }

    /// Note: this the the data under header "resource", not to be confused with the `resourceName`
    /// field inside `protoPayload`
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Resource {
        #[serde(rename = "type")]
        pub resource_type: String,
        pub labels: Labels,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Labels {
        pub project_id: String,
        pub bucket_name: String,
        pub location: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Query {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "GCP_CloudEventsMode")]
        pub cloud_events_mode: Option<String>,
    }
}
