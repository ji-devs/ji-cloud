//! Types for images.

pub mod recent;
pub mod tag;
pub mod user;

use super::{
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ImageStyleId, ImageTagIndex},
    Publish,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
use uuid::Uuid;

/// Represents different kinds of images (which affects how the size is stored in the db)
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum ImageKind {
    /// The image is a canvas (background) image
    Canvas = 0,
    /// The image is a sticker.
    Sticker = 1,
    /// The image is a user profile picture
    UserProfile = 2,
}

impl ImageKind {
    /// The size of a thumbnail (Width x Height pixels).
    pub const THUMBNAIL_SIZE: (u32, u32) = (256, 144);

    /// Gets the proper size of the image once resized.
    #[must_use]
    pub const fn size(self) -> (u32, u32) {
        match self {
            Self::Canvas => (1920, 1080),
            Self::Sticker => (1440, 810),
            Self::UserProfile => (256, 256),
        }
    }

    /// Returns self represented by a string
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Canvas => "Canvas",
            Self::Sticker => "Sticker",
            Self::UserProfile => "UserProfile",
        }
    }
}

/// Wrapper type around [`Uuid`], represents the ID of a image.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ImageId(pub Uuid);

// todo: # errors doc section
/// Request to create a new image.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageCreateRequest {
    /// The name of the image.
    pub name: String,

    /// The description of the image.
    pub description: String,

    /// Is the image premium?
    pub is_premium: bool,

    /// When to publish the image.
    ///
    /// If [`Some`] publish the image according to the `Publish`. Otherwise, don't publish it.
    pub publish_at: Option<Publish>,

    /// The image's styles.
    pub styles: Vec<ImageStyleId>,

    /// The image's age ranges.
    pub age_ranges: Vec<AgeRangeId>,

    /// The image's affiliations.
    pub affiliations: Vec<AffiliationId>,

    /// The image's tags.
    pub tags: Vec<ImageTagIndex>,

    /// The image's categories.
    pub categories: Vec<CategoryId>,

    /// What kind of image this is.
    pub kind: ImageKind,
}

// todo: # errors doc section.
#[derive(Serialize, Deserialize, Debug, Default)]
/// Request to update an image.
///
/// All fields are optional, any field that is [`None`] will not be updated.
pub struct ImageUpdateRequest {
    /// If `Some` change the image's name to this name.
    #[serde(default)]
    pub name: Option<String>,

    /// If `Some` change the image's description to this description.
    #[serde(default)]
    pub description: Option<String>,

    /// If `Some` mark the image as premium or not.
    #[serde(default)]
    pub is_premium: Option<bool>,

    /// If `Some`, change the `publish_at` to the given `Option<Publish>`.
    ///
    /// Specifically, if `None`, don't update.
    /// If `Some(None)`, set the `publish_at` to `None`, unpublishing it if previously published.
    /// Otherwise set it to the given [`Publish`].
    ///
    /// [`Publish`]: struct.Publish.html
    #[serde(deserialize_with = "super::deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub publish_at: Option<Option<Publish>>,

    /// If `Some` replace the image's styles with these.
    #[serde(default)]
    pub styles: Option<Vec<ImageStyleId>>,

    /// If `Some` replace the image's age ranges with these.
    #[serde(default)]
    pub age_ranges: Option<Vec<AgeRangeId>>,

    /// If `Some` replace the image's affiliations with these.
    #[serde(default)]
    pub affiliations: Option<Vec<AffiliationId>>,

    /// If `Some` replace the image's categories with these.
    #[serde(default)]
    pub categories: Option<Vec<CategoryId>>,

    /// If `Some` replace the image's tags with these.
    #[serde(default)]
    pub tags: Option<Vec<ImageTagIndex>>,
}

/// Search for images via the given query string.
///
/// * `kind` field must match the case as represented in the returned json body (`PascalCase`?).
/// * Vector fields, such as `age_ranges` should be given as a comma separated vector (CSV).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ImageSearchQuery {
    /// The query string.
    pub q: String,

    /// Optionally filter by `kind`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ImageKind>,

    /// The page number of the images to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Optionally filter by `image_styles`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<ImageStyleId>,

    /// Optionally filter by `age_ranges`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub age_ranges: Vec<AgeRangeId>,

    /// Optionally filter by `affiliations`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliations: Vec<AffiliationId>,

    /// Optionally filter by `categories`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<CategoryId>,

    /// Optionally order by `tags`, given in decreasing priority.
    ///
    /// # Notes on priority
    /// Consider a request with 4 tags, `[clothing, food, red, sports]`.
    ///
    /// "Priority ordering" means that all items tagged as `clothing` will appear before those
    /// without it, and that `[clothing, food]` will appear before `[clothing]` or `[clothing, red]`.
    ///
    /// ## Assigning scores
    /// The priority is achieved by using Algolia's [filter scoring](https://www.algolia.com/doc/guides/managing-results/refine-results/filtering/in-depth/filter-scoring/) feature with `"sumOrFiltersScore": true`.
    ///
    /// Scores are weighted exponentially by a factor of 2. The lowest priority tag is given a score of 1,
    /// and the `i`th highest priority tag is given a score of `2.pow(i)`. This assignment is *provably*
    /// correct that we get the desired ranking. This can also be interpreted as bit vector with comparison.
    ///
    /// *NOTE*: this means that with `i64` range supported by Algolia, we can only assign priority for
    /// the first 62 tags. The remaining are all given a score of 1.  
    ///
    /// ## Example
    /// For an example request `[clothing, food, red, sports]`, we assign the scores:
    ///
    /// | tag name  | score | (truncated) bit vector score  |
    /// |-----------|-------|-------------------------------|
    /// | clothing  | 8     | `0b_1000`                     |
    /// | food      | 4     | `0b_0100`                     |
    /// | red       | 2     | `0b_0010`                     |
    /// | sports    | 1     | `0b_0001`                     |
    ///
    /// This means that the entries will be returned in the following order, based on their tags:
    ///
    /// | position  | entry name | tag names    | score | (truncated) bit vector score  |
    /// |-----------|------------|--------------|-------|-------------------------------|
    /// | 0         | hat        | clothing     | 8     | `0b_1000`                     |
    /// | 1         | cherry     | red, food    | 6     | `0b_0110`                     |
    /// | 2         | cucumber   | green, food  | 4     | `0b_0100`                     |
    /// | 3         | stop sign  | red          | 2     | `0b_0010`                     |
    /// | 4         | basketball | sports       | 1     | `0b_0001`                     |
    /// | 5         | wallet     | [no tags]    | 0     | `0b_0000`                     |
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_i16_indices")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<ImageTagIndex>,

    /// Optionally filter by `is_premium`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

/// Response for successful search.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageSearchResponse {
    /// the images returned.
    pub images: Vec<ImageResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of images found
    pub total_image_count: u64,
}

/// Query for [`Browse`](crate::api::endpoints::image::Browse).
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageBrowseQuery {
    /// Optionally filter by `is_published`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,

    /// Optionally filter by `kind`
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ImageKind>,

    /// The page number of the images to get.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

/// Response for [`Browse`](crate::api::endpoints::image::Browse).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageBrowseResponse {
    /// the images returned.
    pub images: Vec<ImageResponse>,

    /// The number of pages found.
    pub pages: u32,

    /// The total number of images found
    pub total_image_count: u64,
}

/// Response for getting a single image.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageResponse {
    /// The image metadata.
    pub metadata: ImageMetadata,
}

/// Request to indicate the size of an image for upload.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUploadRequest {
    /// The size of the image to be uploaded in bytes. Allows the API server to check that the file size is
    /// within limits and as a verification at GCS that the entire file was uploaded
    pub file_size: usize,
}

/// URL to upload an image. Supports resumable uploading.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageUploadResponse {
    /// The session URI used for uploading, including the query for uploader ID
    pub session_uri: String,
}

/// Over the wire representation of an image's metadata.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageMetadata {
    /// The image's ID.
    pub id: ImageId,

    /// The name of the image.
    pub name: String,

    /// A string describing the image.
    pub description: String,

    /// Whether or not the image is premium.
    pub is_premium: bool,

    /// What kind of image this is.
    pub kind: ImageKind,

    /// When the image should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// The styles associated with the image.
    pub styles: Vec<ImageStyleId>,

    /// The tags associated with the image.
    pub tags: Vec<ImageTagIndex>,

    /// The age ranges associated with the image.
    pub age_ranges: Vec<AgeRangeId>,

    /// The affiliations associated with the image.
    pub affiliations: Vec<AffiliationId>,

    /// The categories associated with the image.
    pub categories: Vec<CategoryId>,

    /// When the image was originally created.
    pub created_at: DateTime<Utc>,

    /// When the image was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Response for successfully creating a Image.
pub type CreateResponse = super::CreateResponse<ImageId>;

// HACK: we can't get `Vec<_>` directly from the DB, so we have to work around it for now.
// see: https://github.com/launchbadge/sqlx/issues/298
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for ImageMetadata {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbImage {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles,
            age_ranges,
            affiliations,
            categories,
            tags,
            created_at,
            updated_at,
        } = DbImage::from_row(row)?;

        Ok(Self {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles: styles.into_iter().map(|(it,)| it).collect(),
            age_ranges: age_ranges.into_iter().map(|(it,)| it).collect(),
            affiliations: affiliations.into_iter().map(|(it,)| it).collect(),
            categories: categories.into_iter().map(|(it,)| it).collect(),
            tags: tags.into_iter().map(|(it,)| it).collect(),
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbImage {
    pub id: ImageId,
    pub kind: ImageKind,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<(ImageStyleId,)>,
    pub age_ranges: Vec<(AgeRangeId,)>,
    pub affiliations: Vec<(AffiliationId,)>,
    pub categories: Vec<(CategoryId,)>,
    pub tags: Vec<(ImageTagIndex,)>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

into_uuid![ImageId];
