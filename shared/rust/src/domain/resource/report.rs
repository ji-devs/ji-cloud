//! Types for Resource short codes for sharing
use chrono::{DateTime, Utc};
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

use crate::api::endpoints::PathPart;

use super::ResourceId;

/// Wrapper type around [`Uuid`](Uuid), represents the ID of a curation comment.
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, PathPart)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct ReportId(pub Uuid);

make_path_parts!(GetResourceReportPath => "/v1/resource/{}/report/{}" => ResourceId, ReportId);

/// Resource report details
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
pub struct ResourceReport {
    /// Id of report
    pub id: ReportId,

    /// Id of reported resource
    pub resource_id: ResourceId,

    /// Type of     report
    pub report_type: ResourceReportType,

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

make_path_parts!(CreateResourceReportPath => "/v1/resource/{}/report" => ResourceId);

/// Request for reporting a resource
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateResourceReport {
    /// Description of the resource.
    pub report_type: ResourceReportType,
}

/// Request for reporting a resource
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceReportEmail {
    /// Display name of the resource.
    pub display_name: String,

    /// Report type of the report.
    pub report_type: ResourceReportType,

    /// Optional name for reporter
    pub reporter_name: Option<String>,

    /// Optional email of reporter
    pub reporter_email: Option<String>,

    /// Creator name of resource
    pub creator_name: String,
}

/// Type of report
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i16)]
pub enum ResourceReportType {
    #[allow(missing_docs)]
    Offensive = 0,
    #[allow(missing_docs)]
    CopyrightInfringement = 1,
    #[allow(missing_docs)]
    Spam = 2,
    #[allow(missing_docs)]
    Privacy = 3,
    #[allow(missing_docs)]
    ResourceNotPlaying = 4,
    #[allow(missing_docs)]
    Other = 5,
}

impl ResourceReportType {
    #[allow(missing_docs)]
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceReportType::Offensive => "Offensive",
            ResourceReportType::CopyrightInfringement => "Copyright Infringement",
            ResourceReportType::Spam => "Spam",
            ResourceReportType::Privacy => "Privacy",
            ResourceReportType::ResourceNotPlaying => "Resource Can't Be Viewed",
            ResourceReportType::Other => "Other",
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
