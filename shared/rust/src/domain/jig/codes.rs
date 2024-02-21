//! Types for Jig short codes for sharing

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use crate::{api::endpoints::PathPart, domain::module::StableModuleId};

use super::{JigId, JigPlayerSettings};

/// Four-digit code identifying a Jig player session
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PathPart)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigCode(pub i32);

impl ToString for JigCode {
    fn to_string(&self) -> String {
        format!("{:06}", self.0)
    }
}

make_path_parts!(JigPlayerSessionCreatePath => "/v1/jig/codes");

/// Request to create a player session for a jig.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSessionCreateRequest {
    /// ID of the Jig that the session is for
    pub jig_id: JigId,

    /// Display name
    pub name: Option<String>,

    /// Settings for the session
    pub settings: JigPlayerSettings,
}

/// Request to create a player session for a jig.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSessionCreateResponse {
    /// Four-digit code identifying a Jig player session
    pub index: JigCode,
}

/// Over-the-wire representation of a jig player session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigCodeResponse {
    /// Four-digit code identifying a Jig player session
    pub index: JigCode,

    /// Id of Jig this code is for.
    pub jig_id: JigId,

    /// Display name.
    pub name: Option<String>,

    /// Settings for the player session.
    pub settings: JigPlayerSettings,

    /// When the code was created
    pub created_at: DateTime<Utc>,

    /// When the code expires
    pub expires_at: DateTime<Utc>,
}

make_path_parts!(JigCodeListPath => "/v1/jig/codes");

/// Request for jig code list
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigCodeListRequest {
    /// Jig id
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jig_id: Option<JigId>,
}

/// Lists all jig player sessions associated with a jig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigCodeListResponse {
    /// Vector of the jig sessions
    pub codes: Vec<JigCodeResponse>,
}

make_path_parts!(JigCodeSessionsPath => "/v1/jig/codes/{}/sessions" => JigCode);

/// Lists all jig player sessions associated with a jig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigCodeSessionsListResponse {
    /// Vector of the jig sessions
    pub sessions: Vec<JigCodeSessionResponse>,
}

/// Lists all jig player sessions associated with a jig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigCodeSessionResponse {
    /// code
    pub code: JigCode,
    /// Playing's name
    pub players_name: Option<String>,
    /// star time
    pub started_at: DateTime<Utc>,
    /// end time
    pub finished_at: Option<DateTime<Utc>>,
    /// information about the session
    pub info: Option<JigPlaySession>,
}

/// Play session
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct JigPlaySession {
    /// modules
    pub modules: Vec<JigPlaySessionModule>,
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySession {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0;
        let mut earned = 0;
        for module in &self.modules {
            let module_points = module.get_points_earned();
            available += module_points.available;
            earned += module_points.earned;
        }
        PointsEarned { available, earned }
    }
}

/// modules
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum JigPlaySessionModule {
    /// Matching
    Matching(JigPlaySessionMatching),
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionModule {
    fn get_points_earned(&self) -> PointsEarned {
        match self {
            JigPlaySessionModule::Matching(matching) => matching.get_points_earned(),
        }
    }
}

/// get points earned trait
pub trait JigPlaySessionModuleGetPointsEarned {
    /// get points earned method
    fn get_points_earned(&self) -> PointsEarned;
}

/// Jig play session module points earned
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointsEarned {
    /// available points to earn
    pub available: u16,
    /// points actually earned
    pub earned: u16,
}
impl std::fmt::Display for PointsEarned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.earned, self.available)
    }
}
impl PointsEarned {
    /// get percent of points earned
    pub fn percent(&self) -> u16 {
        let output = (self.earned as f32 / self.available as f32) * 100.00;
        output as u16
    }
}

/// matching module
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigPlaySessionMatching {
    /// related module id
    pub stable_module_id: StableModuleId,

    /// list of rounds for this module
    pub rounds: Vec<HashMap<usize, JigPlaySessionMatchingCard>>,
}

impl JigPlaySessionMatching {
    /// create new from module id
    pub fn new(stable_module_id: StableModuleId) -> Self {
        Self {
            stable_module_id,
            rounds: vec![],
        }
    }
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionMatching {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0;
        let mut earned = 0;
        for round in &self.rounds {
            for (_, card) in round {
                available += 2;
                earned += match card.failed_tries {
                    0 => 2,
                    1 => 1,
                    _ => 0,
                };
            }
        }
        PointsEarned { available, earned }
    }
}

///
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct JigPlaySessionMatchingCard {
    /// unsuccessful try count
    pub failed_tries: u16,
}

/// Types for Jig session instance endpoints
pub mod instance {
    use macros::make_path_parts;
    use serde::{Deserialize, Serialize};

    use crate::domain::jig::{
        codes::{JigCode, JigPlaySession},
        JigId, JigPlayerSettings,
    };

    make_path_parts!(PlayerSessionInstanceCreatePath => "/v1/jig/codes/instance");

    /// Request to create a player (who is not the author) session for a JIG.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PlayerSessionInstanceCreateRequest {
        /// Four-digit code identifying a JIG player session
        pub code: JigCode,
    }

    /// Response for successfully creating an instance of a JIG player session. contains the token
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerSessionInstanceResponse {
        /// ID of the JIG that the session is for
        pub jig_id: JigId,

        /// Settings for the player session.
        pub settings: JigPlayerSettings,

        /// Token that will be passed to confirm a JIG was played all the way through
        pub token: String,
    }

    make_path_parts!(PlayerSessionInstanceCompletePath => "/v1/jig/codes/instance/complete");

    /// Request to complete a player session for a JIG.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerSessionInstanceCompleteRequest {
        /// Token that will be passed to confirm a JIG was played all the way through
        pub token: String,

        /// session
        pub session: JigPlaySession,

        /// Playing's name
        pub players_name: Option<String>,
    }
}
