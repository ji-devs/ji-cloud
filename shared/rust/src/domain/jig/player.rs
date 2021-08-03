//! Types for Jig short codes for sharing

use super::JigId;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

/// Settings for the player session.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigPlayerSettings {
    /// Text direction, left-to-right or right-to-left
    pub direction: TextDirection,
    /// Whether or not to display the score
    pub display_score: bool,
    /// Whether or not to track assessments
    pub track_assessments: bool,
    /// Whether or not to enable drag assist
    pub drag_assist: bool,
}

impl Default for JigPlayerSettings {
    fn default() -> Self {
        Self {
            direction: TextDirection::default(),
            display_score: false,
            track_assessments: false,
            drag_assist: false,
        }
    }
}

/// Sets text direction for the jig
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[repr(i16)]
pub enum TextDirection {
    /// left to right
    LeftToRight = 0,
    /// right to left
    RightToLeft = 1,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

/// Request to create a player session for a jig.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigPlayerSessionCreateRequest {
    /// ID of the Jig that the session is for
    pub jig_id: JigId,
    /// Settings for the session
    pub settings: JigPlayerSettings,
}

/// Response for creating or fetching the code associated with a jig.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigPlayerSessionCode {
    /// Four-digit code identifying a Jig player session
    pub index: i16,
}

/// Over-the-wire representation of a player session.
#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct JigPlayerSession {
    /// ID of the Jig that the session is for
    pub jig_id: JigId,
    /// Settings for the player session.
    pub settings: JigPlayerSettings,
}
