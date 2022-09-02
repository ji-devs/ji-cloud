//! types for searching

use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

make_path_parts!(CreateSearchKeyPath => "/v1/search/key");

/// Represents the response given when an api key for algolia is requested.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateSearchKeyResponse {
    /// The key to be used with algolia
    pub key: String,
    // todo: add the expire time
}

make_path_parts!(WebImageSearchPath => "/v1/search/web/image");

/// Search for images via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebImageSearchQuery {
    /// The query string.
    #[serde(default)]
    pub q: String,

    /// Image type string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<ImageType>,
}

/// Represents different types of images
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum ImageType {
    /// Animated Gif Images
    Clipart = 0,
    /// Clip art images
    AnimatedGif = 1,
    /// Photographs (excluding line drawings, animated gifs, and clip art)
    Photo = 2,
    /// Line drawings
    Line = 3,
    /// Images with transparent backgrounds
    Transparent = 4,
}

/// Image types used in query string
impl ImageType {
    ///
    #[must_use]
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Clipart => "Clipart",
            Self::AnimatedGif => "AnimatedGif",
            Self::Photo => "Photo",
            Self::Line => "Line",
            Self::Transparent => "Transparent",
        }
    }
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
