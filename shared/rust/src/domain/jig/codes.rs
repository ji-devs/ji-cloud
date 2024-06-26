//! Types for Jig short codes for sharing

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};

use crate::{api::endpoints::PathPart, domain::module::StableModuleId};

use super::{JigId, JigPlayerSettings, JigResponse};

/// Four-digit code identifying a Jig player session
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PathPart, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct JigCode(pub i32);

impl ToString for JigCode {
    fn to_string(&self) -> String {
        format!("{:06}", self.0)
    }
}

make_path_parts!(JigPlayerSessionCreatePath => "/v1/jig/codes");

/// Request to create a jig code.
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

/// Response from creating a jig code.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JigPlayerSessionCreateResponse {
    /// Four-digit code identifying a Jig player session
    pub index: JigCode,
}

make_path_parts!(JigCodePath => "/v1/jig/codes/{}" => JigCode);

/// Request to update a jig code.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct JigCodeUpdateRequest {
    /// Display name
    pub name: Option<Option<String>>,

    /// Settings for the session
    pub settings: Option<JigPlayerSettings>,
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
    /// Vector of the jig codes
    pub codes: Vec<JigCodeResponse>,
}

make_path_parts!(JigsWithCodesPath => "/v1/jig/codes/jig-codes");

/// Lists all jig player sessions associated with a jig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigsWithCodesResponse {
    /// Vector of the jig that have jig codes
    pub jigs: Vec<JigWithCodes>,
}

/// Jig with codes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JigWithCodes {
    /// jig
    pub jig: JigResponse,
    /// codes
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
    #[serde(default)]
    pub modules: Vec<JigPlaySessionModule>,
    /// Modules just visited
    #[serde(default)]
    pub visited: HashSet<StableModuleId>,
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySession {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0.0;
        let mut earned = 0.0;
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
    /// Card quiz
    CardQuiz(JigPlaySessionCardQuiz),
    /// Drag and drop
    DragDrop(JigPlaySessionDragDrop),
    /// Answer this
    FindAnswer(JigPlaySessionFindAnswer),
}

impl JigPlaySessionModule {
    /// get stable module id
    pub fn stable_module_id(&self) -> StableModuleId {
        match self {
            Self::Matching(module) => module.stable_module_id,
            Self::CardQuiz(module) => module.stable_module_id,
            Self::DragDrop(module) => module.stable_module_id,
            Self::FindAnswer(module) => module.stable_module_id,
        }
    }
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionModule {
    fn get_points_earned(&self) -> PointsEarned {
        match self {
            JigPlaySessionModule::Matching(module) => module.get_points_earned(),
            JigPlaySessionModule::CardQuiz(module) => module.get_points_earned(),
            JigPlaySessionModule::DragDrop(module) => module.get_points_earned(),
            JigPlaySessionModule::FindAnswer(module) => module.get_points_earned(),
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
    pub available: f32,
    /// points actually earned
    pub earned: f32,
}
impl std::fmt::Display for PointsEarned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.earned, self.available)
    }
}
impl PointsEarned {
    /// get percent of points earned
    pub fn percent(&self) -> u16 {
        let output = (self.earned / self.available) * 100.00;
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
        let mut available = 0.0;
        let mut earned = 0.0;
        for round in &self.rounds {
            for (_, card) in round {
                available += 1.0;
                earned += match card.failed_tries {
                    0 => 1.0,
                    1 => 0.5,
                    _ => 0.0,
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

/// CardQuiz module
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigPlaySessionCardQuiz {
    /// related module id
    pub stable_module_id: StableModuleId,

    /// list of rounds for this module
    pub rounds: Vec<JigPlaySessionCardQuizRound>,
}

impl JigPlaySessionCardQuiz {
    /// create new from module id
    pub fn new(stable_module_id: StableModuleId) -> Self {
        Self {
            stable_module_id,
            rounds: Vec::new(),
        }
    }
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionCardQuiz {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0.0;
        let mut earned = 0.0;
        for card in &self.rounds {
            available += 1.0;
            earned += match card.failed_tries {
                0 => 1.0,
                1 => 0.5,
                _ => 0.0,
            };
        }
        PointsEarned { available, earned }
    }
}

///
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct JigPlaySessionCardQuizRound {
    /// index of card
    pub card_index: usize,

    /// unsuccessful try count
    pub failed_tries: u16,
}

/// Drag and drop module
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigPlaySessionDragDrop {
    /// related module id
    pub stable_module_id: StableModuleId,

    /// list of rounds for this module. key is index in module.items.
    /// HashMap instead of Vec because not all items in modules.items are interactive.
    pub items: HashMap<usize, JigPlaySessionDragDropItem>,
}

impl JigPlaySessionDragDrop {
    /// create new from module id
    pub fn new(stable_module_id: StableModuleId) -> Self {
        Self {
            stable_module_id,
            items: HashMap::new(),
        }
    }
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionDragDrop {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0.0;
        let mut earned = 0.0;
        for card in self.items.values() {
            available += 1.0;
            earned += match card.failed_tries {
                0 => 1.0,
                1 => 0.5,
                _ => 0.0,
            };
        }
        PointsEarned { available, earned }
    }
}

///
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct JigPlaySessionDragDropItem {
    /// unsuccessful try count
    pub failed_tries: u16,
}

/// Drag and drop module
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigPlaySessionFindAnswer {
    /// related module id
    pub stable_module_id: StableModuleId,

    /// list of rounds for this module
    pub items: Vec<JigPlaySessionFindAnswerItem>,
}

impl JigPlaySessionFindAnswer {
    /// create new from module id
    pub fn new(stable_module_id: StableModuleId) -> Self {
        Self {
            stable_module_id,
            items: Vec::new(),
        }
    }
}

impl JigPlaySessionModuleGetPointsEarned for JigPlaySessionFindAnswer {
    fn get_points_earned(&self) -> PointsEarned {
        let mut available = 0.0;
        let mut earned = 0.0;
        for card in &self.items {
            available += 1.0;
            earned += match card.failed_tries {
                0 => 1.0,
                1 => 0.5,
                _ => 0.0,
            };
        }
        PointsEarned { available, earned }
    }
}

///
#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct JigPlaySessionFindAnswerItem {
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
