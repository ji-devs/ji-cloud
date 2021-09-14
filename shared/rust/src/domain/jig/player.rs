//! Types for Jig short codes for sharing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::JigId;

/// Settings for the player session.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSettings {
    /// Text direction, left-to-right or right-to-left
    #[serde(default)]
    pub direction: TextDirection,
    /// Whether or not to display the score
    #[serde(default)]
    pub display_score: bool,
    /// Whether or not to track assessments
    #[serde(default)]
    pub track_assessments: bool,
    /// Whether or not to enable drag assist
    #[serde(default)]
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

/// Sets text direction for the jig.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[repr(i16)]
pub enum TextDirection {
    /// left to right
    #[serde(rename = "ltr")]
    LeftToRight = 0,

    /// right to left
    #[serde(rename = "rtl")]
    RightToLeft = 1,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::LeftToRight
    }
}

/// Four-digit code identifying a Jig player session
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigPlayerSessionIndex(pub i16);

/// Request to create a player session for a jig.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSessionCreateRequest {
    /// ID of the Jig that the session is for
    pub jig_id: JigId,

    /// Settings for the session
    pub settings: JigPlayerSettings,
}

/// Request to create a player session for a jig.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSessionCreateResponse {
    /// Four-digit code identifying a Jig player session
    pub index: JigPlayerSessionIndex,
}

/// Over-the-wire representation of a jig player session
#[derive(Serialize, Deserialize, Debug)]
pub struct JigPlayerSession {
    /// Four-digit code identifying a Jig player session
    pub index: JigPlayerSessionIndex,

    /// Settings for the player session.
    pub settings: JigPlayerSettings,

    /// When the player session expires
    pub expires_at: DateTime<Utc>,
}

/// Lists all jig player sessions associated with a jig
#[derive(Serialize, Deserialize, Debug)]
pub struct JigPlayerSessionListResponse {
    /// Vector of the jig sessions
    pub sessions: Vec<JigPlayerSession>,
}

/// Response for completing a session for a jig play as a player and updating the jig play count
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayCountResponse {
    /// Number of times a jig was completed
    pub play_count: i64,
}

/// Types for Jig session instance endpoints
pub mod instance {
    use serde::{Deserialize, Serialize};

    use crate::domain::jig::{player::JigPlayerSessionIndex, JigId, JigPlayerSettings};

    /// Request to create a player (who is not the author) session for a JIG.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayerSessionInstanceCreateRequest {
        /// Four-digit code identifying a JIG player session
        pub index: JigPlayerSessionIndex,
    }

    /// Response for successfully creating an instance of a JIG player session. contains the token
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerSessionInstanceResponse {
        /// ID of the JIG that the session is for
        pub jig_id: JigId,

        /// Settings for the player session.
        pub settings: JigPlayerSettings,

        /// Token that will be passed to confirm a JIG was played all the way through
        pub token: String,
    }

    /// Request to complete a player session for a JIG.
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerSessionInstanceCompleteRequest {
        /// Token that will be passed to confirm a JIG was played all the way through
        pub token: String,
    }
}
