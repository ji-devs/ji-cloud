//! types for searching

use serde::{Deserialize, Serialize};

/// Represents the response given when an api key for algolia is requested.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSearchKeyResponse {
    /// The key to be used with algolia
    pub key: String,
    // todo: add the expire time
}

/// Search for images via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WebImageSearchQuery {
    /// The query string.
    #[serde(default)]
    pub q: String,

    /// Image type string
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
}

/// A single image as returned from the web
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebImageSearchItem {
    /// A URL to the thumbnail of the image.
    pub thumbnail_url: url::Url,
    /// A URL to the original image.
    pub url: url::Url,
}

/// Response for successful search.
/// TODO: support pagation
#[derive(Serialize, Deserialize, Debug)]
pub struct WebImageSearchResponse {
    /// the images returned.
    pub images: Vec<WebImageSearchItem>,
}
