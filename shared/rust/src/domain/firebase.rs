//! TODO this

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages
#[derive(Deserialize, Serialize)]
pub struct FirebaseCloudMessage {
    /// Name of the message.
    pub name: Option<String>,
    /// Data field.
    pub data: HashMap<String, String>,
    /// The target of the message, one of `Token`, `Topic`, or `Condition`
    #[serde(flatten)]
    pub target: MessageTarget,
}

/// Possible ways to specify the target
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageTarget {
    /// Specify push notification using a client's token.
    Token(String),
    /// Publish to a topic by name.
    Topic(String),
    /// Publish to a combination of topics, with boolean.
    /// See for usage: https://firebase.google.com/docs/cloud-messaging/android/send-multiple
    Condition(String),
}

/// Struct representing error response from Firebase.
#[derive(Deserialize, Serialize)]
struct FcmError {
    /// Error code returned by Firebase.
    error_code: FcmErrorReason,
}

/// Possible error reasons.
/// See: https://firebase.google.com/docs/reference/fcm/rest/v1/ErrorCode
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(untagged)]
#[allow(missing_docs)]
#[repr(i16)]
enum FcmErrorReason {
    InvalidArgument = 400,
    Unregistered = 404,
    SenderIdMismatch = 403,
    QuotaExceeded = 429,
    Unavailable = 503,
    Internal = 500,
    ThirdPartyError = 401,
}
