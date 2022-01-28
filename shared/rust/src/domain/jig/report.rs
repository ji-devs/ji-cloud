//! Types for Jig short codes for sharing
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

use super::JigId;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a curation comment.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ReportId(pub Uuid);

/// Jig report details
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
pub struct JigReport {
    /// Id of report
    pub id: ReportId,

    /// Id of reported jig
    pub jig_id: JigId,

    /// Type of     report
    pub report_type: JigReportType,

    /// Optional id of reporter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_id: Option<Uuid>,

    /// Optional name for reporter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_name: Option<String>,

    /// Optional email of reporter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_email: Option<String>,

    /// When report was submitted
    pub created_at: DateTime<Utc>,
}

/// Request for reporting a jig
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateJigReport {
    /// Description of the jig.
    pub report_type: JigReportType,
}

/// Request for reporting a jig
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JigReportEmail {
    /// Display name of the jig.
    pub display_name: String,

    /// Report type of the report.
    pub report_type: JigReportType,

    /// Optional name for reporter
    pub reporter_name: Option<String>,

    /// Optional email of reporter
    pub reporter_email: Option<String>,

    /// Creator name of jig
    pub creator_name: String,
}

/// Type of report
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum JigReportType {
    #[allow(missing_docs)]
    Offensive = 0,
    #[allow(missing_docs)]
    CopyrightInfringement = 1,
    #[allow(missing_docs)]
    Spam = 2,
    #[allow(missing_docs)]
    Privacy = 3,
    #[allow(missing_docs)]
    JiTapGameNotPlaying = 4,
    #[allow(missing_docs)]
    Other = 5,
}

impl JigReportType {
    #[allow(missing_docs)]
    pub fn as_str(&self) -> &'static str {
        match self {
            JigReportType::Offensive => "Offensive",
            JigReportType::CopyrightInfringement => "Copyright Infringement",
            JigReportType::Spam => "Spam",
            JigReportType::Privacy => "Privacy",
            JigReportType::JiTapGameNotPlaying => "Ji Tap Game Not Playing",
            JigReportType::Other => "Other",
        }
    }

    #[allow(missing_docs)]
    pub fn to_value_str(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    #[allow(missing_docs)]
    pub fn from_value_str(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

into_uuid![ReportId];
