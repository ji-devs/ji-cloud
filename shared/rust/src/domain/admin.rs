//! Types for admin routes.
use crate::api::endpoints::PathPart;
use crate::domain::billing::{Account, AccountUser, AdminSchool, SchoolNameId};
use crate::domain::user::UserId;
use crate::domain::{billing::SchoolId, ItemCount, Page, PageLimit};
use chrono::Utc;
use macros::make_path_parts;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

make_path_parts!(AdminUserExportPath => "/v1/admin/export/users");
/// Request to export data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminUserExportRequest {
    ///
    pub date_filter_type: DateFilterType,
    /// Optionally the date to export data from
    pub from_date: Option<chrono::DateTime<Utc>>,
    /// Optionally the date to export data to
    pub to_date: Option<chrono::DateTime<Utc>>,
}

make_path_parts!(AdminJigExportPath => "/v1/admin/export/jigs");

make_path_parts!(AdminPlaylistExportPath => "/v1/admin/export/playlists");

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

make_path_parts!(AdminSchoolsPath => "/v1/admin/schools");

/// Request to list school names
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SearchSchoolsParams {
    /// String to search school names by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// If `Some` then whether to filter by verified or unverified, otherwise return all schools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Current page of results
    #[serde(default)]
    pub page: Page,
    /// Total schools per page to return
    #[serde(default)]
    pub page_limit: PageLimit,
}

/// List of school names and their associated schools
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchSchoolsResponse {
    /// List of schools
    pub schools: Vec<AdminSchool>,
    /// Count of pages
    pub pages: ItemCount,
    /// Total count of schools for this query
    pub total_schools_count: ItemCount,
}

make_path_parts!(AdminSchoolAccountPath => "/v1/admin/schools/{}" => SchoolId);

/// Request to create a new school account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAdminSchoolAccountResponse {
    /// School name
    pub school: AdminSchool,
    /// Account associated with the school
    pub account: Account,
    /// School location
    pub users: Vec<AccountUser>,
}

make_path_parts!(AdminVerifySchoolPath => "/v1/admin/schools/verify");

/// Request to update verification of a `SchoolName`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifySchoolRequest {
    /// The ID of the school to update verification
    pub school_id: SchoolId,
    /// Whether this school should be marked verified or not
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
    /// The user hasn't completed setting up their profile
    #[strum(serialize = "Incomplete profile")]
    IncompleteProfile,
}

make_path_parts!(SchoolNamesPath => "/v1/admin/school-names");

make_path_parts!(UpdateSchoolNamePath => "/v1/admin/school-names/{}" => SchoolNameId);

make_path_parts!(SetInternalSchoolNamePath => "/v1/admin/schools/{}/school-name" => SchoolId);

make_path_parts!(SetAccountTierOverridePath => "/v1/admin/users/{}/tier-override" => UserId);

make_path_parts!(DeleteUserAccountPath => "/v1/admin/users/{}/account" => UserId);

make_path_parts!(RemoveUserFromSchoolPath => "/v1/admin/schools/{}/users" => SchoolId);
