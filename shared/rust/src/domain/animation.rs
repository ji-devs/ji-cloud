//! Types for animations.

#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper type around [`Uuid`], represents the ID of an animation.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct AnimationId(pub Uuid);

into_uuid![AnimationId];
