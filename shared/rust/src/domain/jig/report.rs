//! Types for Jig short codes for sharing
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::JigId;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a curation comment.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ReportId(pub Uuid);

/// Request for reporting a jig
#[derive(Serialize, Deserialize, Clone, Debug)]
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

/// Type of report
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
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
    Other = 4,
}

into_uuid![ReportId];
