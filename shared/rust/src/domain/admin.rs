//! Types for admin routes.
use crate::domain::{
    billing::{School, SchoolId, SchoolName, SchoolNameId},
    ItemCount, Page, PageLimit,
};
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

make_path_parts!(AdminSchoolNamesPath => "/v1/admin/school-name");

/// Request to list school names
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SearchSchoolNamesParams {
    /// String to search school names by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// If `Some` then whether to filter by verified or unverified, otherwise return all school names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Current page of results
    #[serde(default)]
    pub page: Page,
    /// Total schools per page to return
    #[serde(default)]
    pub page_limit: PageLimit,
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
pub struct SearchSchoolNamesResponse {
    /// List of school names and their associated school account if one exists
    pub school_names: Vec<SchoolNameUsageResponse>,
    /// Count of pages
    pub pages: ItemCount,
    /// Total count of schools for this query
    pub total_schools_count: ItemCount,
}

make_path_parts!(AdminVerifySchoolNamePath => "/v1/admin/school-name/verify");

/// Request to update verification of a `SchoolName`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifySchoolNameRequest {
    /// The ID of the school name to update verification
    pub school_name_id: SchoolNameId,
    /// Whether this school name should be marked verified or not
    pub verified: bool,
}

make_path_parts!(ImportSchoolNamesPath => "/v1/admin/import-school-names");

make_path_parts!(InviteSchoolUsersPath => "/v1/admin/invite-users");

/// Request to invite users to a school by ID. The data is a newline separated list
/// of user emails.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteSchoolUsersRequest {
    /// School ID to invite users to
    pub school_id: SchoolId,
    /// Newline-separated list of user emails
    pub data: String,
}

/// Response holding list of failed emails and the reasons
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteSchoolUsersResponse {
    /// List of failed invites
    pub failures: Vec<InviteSchoolUserFailure>,
}

/// Represents a failed invited user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InviteSchoolUserFailure {
    /// The users email
    pub email: String,
    /// The reason the user could not be associated
    pub reason: InviteFailedReason,
}

/// Possible invite failure reasons
#[derive(Display, Serialize, Deserialize, Debug, Clone)]
pub enum InviteFailedReason {
    /// The user already has an individual account
    #[strum(serialize = "Has individual account")]
    HasIndividualAccount,
    /// The user is already associated with another school
    #[strum(serialize = "Associated with another school")]
    AssociatedWithSchool,
    /// The user could not be found (not registered yet)
    #[strum(serialize = "Not found")]
    UserNotFound,
}
