//! Types for admin routes.
use chrono::Utc;
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Type of data export to perform
#[derive(Display, EnumIter, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportType {
    /// Export user profiles
    Profiles,
}

make_path_parts!(ExportDataPath => "/v1/admin/export");

/// Request to export data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportDataRequest {
    /// The type of data to export
    pub export_type: ExportType,
    /// Optionally the date to export data from
    pub from_date: Option<chrono::DateTime<Utc>>,
    /// Optionally the date to export data to
    pub to_date: Option<chrono::DateTime<Utc>>,
}
