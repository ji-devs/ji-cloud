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
    #[strum(serialize = "User profiles")]
    Profiles,
}

impl Default for ExportType {
    fn default() -> Self {
        Self::Profiles
    }
}

make_path_parts!(ExportDataPath => "/v1/admin/export");

/// Request to export data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExportDataRequest {
    /// The type of data to export
    pub export_type: ExportType,
    ///
    pub date_filter_type: DateFilterType,
    /// Optionally the date to export data from
    pub from_date: Option<chrono::DateTime<Utc>>,
    /// Optionally the date to export data to
    pub to_date: Option<chrono::DateTime<Utc>>,
}

/// Type of filter to apply for the date ranges
#[derive(Display, EnumIter, Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DateFilterType {
    /// Only filter on new records
    #[strum(serialize = "Only new records")]
    OnlyNew,
    /// Only filter on updated records
    #[strum(serialize = "Only updated records")]
    OnlyUpdated,
    /// Filter by either new or updated records
    #[strum(serialize = "New or updated records")]
    Either,
}

impl Default for DateFilterType {
    fn default() -> Self {
        Self::OnlyNew
    }
}
