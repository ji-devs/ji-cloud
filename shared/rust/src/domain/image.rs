//! Types for images.

use super::{
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ImageStyleId, TagId},
    Publish,
};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
use uuid::Uuid;

/// Types for user image library.
pub mod user {
    #[cfg(feature = "backend")]
    use paperclip::actix::Apiv2Schema;
    use serde::{Deserialize, Serialize};

    use super::ImageId;

    /// Response for listing.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserImageListResponse {
        /// the images returned.
        pub images: Vec<UserImageResponse>,
    }

    /// Response for getting a single image.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserImageResponse {
        /// The image metadata.
        pub metadata: UserImage,
    }

    /// Over the wire representation of an image's metadata.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserImage {
        /// The image's ID.
        pub id: ImageId,
        // more fields to be added
    }
}

/// Types to manage image tags.
pub mod tag {
    use crate::domain::meta::TagId;
    #[cfg(feature = "backend")]
    use paperclip::actix::Apiv2Schema;
    use serde::{Deserialize, Serialize};

    /// Request to create an image tag.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct ImageTagCreateRequest {
        /// Display name of the image tag.
        pub display_name: String,
    }

    /// Response returned to list all image tags.
    #[derive(Serialize, Deserialize)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct ImageTagListResponse {
        /// Indexes and ids for all the image tags.
        pub image_tags: Vec<ImageTagResponse>,
    }

    /// Response for a single tag.
    #[derive(Serialize, Deserialize)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct ImageTagResponse {
        /// The index of the image tag found.
        pub index: i16,

        /// The display name of the image tag found.
        pub display_name: String,

        /// The id of the image tag found.
        pub id: TagId,
    }

    /// Request to update an image tag.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct ImageTagUpdateRequest {
        /// Display name of the image tag. `None` means no change to be made.
        pub display_name: Option<String>,

        /// If [`Some`] attempt to move tag to the given index. If it is already occupied, do no
        /// change the indexing.
        ///
        /// If `index` is [`None`] then it will not be updated.
        pub index: Option<i16>,
    }
}

/// Types for a user's recent images list. Can be from any ['MediaLibrary'](crate::media::MediaLibrary).
/// Does not verify entries for validity/existence.
pub mod recent {
    use super::ImageId;
    use crate::media::MediaLibrary;
    use chrono::{DateTime, Utc};
    #[cfg(feature = "backend")]
    use paperclip::actix::Apiv2Schema;
    use serde::{Deserialize, Serialize};

    /// Over-the-wire representation of a single recent image.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserRecentImageResponse {
        /// The image's ID.
        pub id: ImageId,

        /// The library that the image belongs to.
        pub library: MediaLibrary,

        /// When the image was last used.
        pub last_used: DateTime<Utc>,
    }

    /// Request to add an entry to the recent user images list,
    /// see ['recent::Create'](crate::endpoints::image::recent::Create).
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserRecentImageCreateRequest {
        /// The image's ID.
        pub id: ImageId,

        /// The library that the image belongs to.
        pub library: MediaLibrary,
    }

    /// Query to list a user's recent images,
    /// see ['recent::List'](crate::endpoints::image::recent::List).
    ///
    /// This query is optional.
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserRecentImageListRequest {
        /// Indicates how many recent items to retrieve.
        pub limit: u16,
    }

    /// Response for listing a user's recent images,
    /// see ['recent::List'](crate::endpoints::image::recent::List).
    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "backend", derive(Apiv2Schema))]
    pub struct UserRecentImageListResponse {
        /// The images returned.
        pub images: Vec<UserRecentImageResponse>,
    }
}

/// Represents different kinds of images (which affects how the size is stored in the db)
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
pub enum ImageKind {
    /// The image is a canvas (background) image
    Canvas = 0,
    /// The image is a sticker.
    Sticker = 1,
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
        }
    }

    /// Returns self represented by a string
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Canvas => "Canvas",
            Self::Sticker => "Sticker",
        }
    }
}

/// Wrapper type around [`Uuid`], represents the ID of a image.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageId(pub Uuid);

// todo: # errors doc section
/// Request to create a new image.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
    pub tags: Vec<TagId>,

    /// The image's categories.
    pub categories: Vec<CategoryId>,

    /// What kind of image this is.
    pub kind: ImageKind,
}

// todo: # errors doc section.
#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
    pub tags: Option<Vec<TagId>>,
}

/// Search for images via the given query string.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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

    /// Optionally filter by `tags`
    #[serde(default)]
    #[serde(serialize_with = "super::csv_encode_uuids")]
    #[serde(deserialize_with = "super::from_csv")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<TagId>,

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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct ImageResponse {
    /// The image metadata.
    pub metadata: ImageMetadata,
}

/// Over the wire representation of an image's metadata.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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
    pub tags: Vec<TagId>,

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

/// Response for successfuly creating a Image.
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
    pub tags: Vec<(TagId,)>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

into_uuid![ImageId];
