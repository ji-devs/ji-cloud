//! types for searching

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;

/// Represents the response given when an api key for algolia is requested.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CreateSearchKeyResponse {
    /// The key to be used with algolia
    pub key: String,
    // todo: add the expire time
}
