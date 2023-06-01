//! Types for admin routes.
use crate::domain::billing::{School, SchoolName, SchoolNameId};
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

make_path_parts!(SchoolNameVerification => "/v1/admin/school-name");

/// Request to list school names
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSchoolNamesRequest {
    /// If `Some` then whether to filter by verified or unverified
    pub verified: Option<bool>,
}

/// A school name and it's usage in a School
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SchoolNameUsageResponse {
    /// A school name
    pub school_name: SchoolName,
    /// An optional usage of a school name
    pub school: Option<School>,
}

impl From<(SchoolName, Option<School>)> for SchoolNameUsageResponse {
    fn from((school_name, school): (SchoolName, Option<School>)) -> Self {
        Self {
            school_name,
            school,
        }
    }
}

/// List of school names and their associated schools
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListSchoolNamesResponse {
    /// List of school names and their associated school account if one exists
    pub school_names: Vec<SchoolNameUsageResponse>,
}

/// Request to update verification of a `SchoolName`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifySchoolNameRequest {
    /// The ID of the school name to update verification
    pub school_name_id: SchoolNameId,
    /// Whether this school name should be marked verified or not
    pub verified: bool,
}
