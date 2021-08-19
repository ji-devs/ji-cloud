//! Types for animations.

use super::{meta::AnimationStyleId, Publish};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
use uuid::Uuid;

/// Animation Kinds
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum AnimationKind {
    /// Gif Animation
    Gif = 0,
    /// Spritesheet Animation
    Spritesheet = 1,
}

impl AnimationKind {
    /// returns `self` in a string representation.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Gif => "gif",
            Self::Spritesheet => "spritesheet",
        }
    }
}

/// Wrapper type around [`Uuid`], represents the ID of an animation.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AnimationId(pub Uuid);

/// Response for getting a single animation file.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationResponse {
    /// The animation's metadata.
    pub metadata: AnimationMetadata,
}

/// Over the wire representation of an animation's metadata.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationMetadata {
    /// The animation's ID.
    pub id: AnimationId,

    /// The name of the animation.
    pub name: String,

    /// The description of the animation.
    pub description: String,

    /// Is the animation premium?
    pub is_premium: bool,

    /// When the animation should be considered published (if at all).
    pub publish_at: Option<DateTime<Utc>>,

    /// The styles associated with the animation.
    pub styles: Vec<AnimationStyleId>,

    /// What kind of animation this is.
    pub kind: AnimationKind,

    /// Should the animation loop?
    pub is_looping: bool,

    /// When the animation was originally created.
    pub created_at: DateTime<Utc>,

    /// When the animation was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

// HACK: can't get `Vec<_>` directly from the DB. See `[crate::domain::image::ImageMetadata]`
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for AnimationMetadata {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbAnimation {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles,
            is_looping,
            created_at,
            updated_at,
        } = DbAnimation::from_row(row)?;

        Ok(Self {
            id,
            kind,
            name,
            description,
            is_premium,
            publish_at,
            styles: styles.into_iter().map(|(it,)| it).collect(),
            is_looping,
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[cfg(feature = "backend")]
struct DbAnimation {
    pub id: AnimationId,
    pub kind: AnimationKind,
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub publish_at: Option<DateTime<Utc>>,
    pub styles: Vec<(AnimationStyleId,)>,
    pub is_looping: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// todo: # errors doc section
/// Request to create a new animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationCreateRequest {
    /// The name of the animation.
    pub name: String,

    /// The description of the animation.
    pub description: String,

    /// Is the animation premium?
    pub is_premium: bool,

    /// When to publish the animation.
    ///
    /// If [`Some`] publish the animation according to the `Publish`. Otherwise, don't publish it.
    pub publish_at: Option<Publish>,

    /// The styles associated with the animation.
    pub styles: Vec<AnimationStyleId>,

    /// What kind of animation this is.
    pub kind: AnimationKind,

    /// Should the animation loop?
    pub is_looping: bool,
}

/// Request to indicate the size of an user library image for upload.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationUploadRequest {
    /// The size of the image to be uploaded in bytes.
    pub file_size: usize,
}

/// URL to upload an user library image, supports resumable uploading.
#[derive(Serialize, Deserialize, Debug)]
pub struct AnimationUploadResponse {
    /// The session URI used for uploading, including the query for uploader ID
    pub session_uri: String,
}

into_uuid![AnimationId];
