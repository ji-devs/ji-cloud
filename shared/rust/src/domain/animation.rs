//! Types for animations.

use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Publish;

/// Animation Variants
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", derive(paperclip::actix::Apiv2Schema))]
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
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AnimationId(pub Uuid);

/// Response for getting a single audio file.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AnimationResponse {
    /// The animation's metadata.
    pub metadata: AnimationMetadata,
}

/// Over the wire representation of an animation's metadata.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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

    /// What kind of animation this is.
    pub kind: AnimationKind,

    /// Should the animation loop?
    pub is_looping: bool,

    /// When the animation was originally created.
    pub created_at: DateTime<Utc>,

    /// When the animation was last updated.
    pub updated_at: Option<DateTime<Utc>>,
}

// todo: # errors doc section
/// Request to create a new animation.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
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

    /// What kind of animation this is.
    pub variant: AnimationKind,

    /// Should the animation loop?
    pub is_looping: bool,
}

into_uuid![AnimationId];
