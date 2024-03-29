use super::ApiEndpoint;
use crate::domain::admin::{
    AdminSchoolAccountPath, AdminSchoolsPath, AdminVerifySchoolPath, DeleteUserAccountPath,
    GetAdminSchoolAccountResponse, ImportSchoolNamesPath, InviteSchoolUsersPath,
    InviteSchoolUsersRequest, InviteSchoolUsersResponse, RemoveUserFromSchoolPath, SchoolNamesPath,
    SearchSchoolsParams, SearchSchoolsResponse, SetAccountTierOverridePath,
    SetInternalSchoolNamePath, UpdateSchoolNamePath, VerifySchoolRequest,
};
use crate::domain::billing::{PlanTier, SchoolName, SchoolNameId, SchoolNameValue};
use crate::domain::user::UserId;
use crate::domain::UpdateNullable;
use crate::error::AccountError;
use crate::{
    api::Method,
    domain::{
        admin::{
            AdminJigExportPath, AdminPlaylistExportPath, AdminUserExportPath,
            AdminUserExportRequest,
        },
        billing::{SubscriptionPlanPath, UpdateSubscriptionPlansRequest},
        session::{ImpersonatePath, NewSessionResponse},
    },
    error::EmptyError,
};

/// Impersonate another user.
pub struct Impersonate;
impl ApiEndpoint for Impersonate {
    type Path = ImpersonatePath;
    type Req = ();
    type Res = NewSessionResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Export user data
pub struct AdminUserExport;
impl ApiEndpoint for AdminUserExport {
    type Path = AdminUserExportPath;
    type Req = AdminUserExportRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Export jig data
pub struct AdminJigExport;
impl ApiEndpoint for AdminJigExport {
    type Path = AdminJigExportPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Export playlist data
pub struct AdminPlaylistExport;
impl ApiEndpoint for AdminPlaylistExport {
    type Path = AdminPlaylistExportPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create or update a subscription plan
pub struct CreateUpdateSubscriptionPlans;
impl ApiEndpoint for CreateUpdateSubscriptionPlans {
    type Path = SubscriptionPlanPath;
    type Req = UpdateSubscriptionPlansRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// List school names
pub struct SearchSchools;
impl ApiEndpoint for SearchSchools {
    type Path = AdminSchoolsPath;
    type Req = SearchSchoolsParams;
    type Res = SearchSchoolsResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a school names verification flag
pub struct VerifySchool;
impl ApiEndpoint for VerifySchool {
    type Path = AdminVerifySchoolPath;
    type Req = VerifySchoolRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Update a school names verification flag
pub struct ImportSchoolNames;
impl ApiEndpoint for ImportSchoolNames {
    type Path = ImportSchoolNamesPath;
    type Req = String;
    type Res = Vec<String>;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Invite users to a school
pub struct InviteUsers;
impl ApiEndpoint for InviteUsers {
    type Path = InviteSchoolUsersPath;
    type Req = InviteSchoolUsersRequest;
    type Res = InviteSchoolUsersResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get a school account for administration
pub struct GetAdminSchoolAccount;
impl ApiEndpoint for GetAdminSchoolAccount {
    type Path = AdminSchoolAccountPath;
    type Req = ();
    type Res = GetAdminSchoolAccountResponse;
    type Err = AccountError;
    const METHOD: Method = Method::Get;
}

/// Set an accounts plan tier override
pub struct SetAccountTierOverride;
impl ApiEndpoint for SetAccountTierOverride {
    type Path = SetAccountTierOverridePath;
    type Req = UpdateNullable<PlanTier>;
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Patch;
}

/// Get a list of imported school names
pub struct GetSchoolNames;
impl ApiEndpoint for GetSchoolNames {
    type Path = SchoolNamesPath;
    type Req = ();
    type Res = Vec<SchoolName>;
    type Err = AccountError;
    const METHOD: Method = Method::Get;
}

/// Update a school name
pub struct UpdateSchoolName;
impl ApiEndpoint for UpdateSchoolName {
    type Path = UpdateSchoolNamePath;
    type Req = SchoolNameValue;
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Patch;
}

/// Create a school name
pub struct CreateSchoolName;
impl ApiEndpoint for CreateSchoolName {
    type Path = SchoolNamesPath;
    type Req = SchoolNameValue;
    type Res = SchoolNameId;
    type Err = AccountError;
    const METHOD: Method = Method::Post;
}
/// Set a school name for a school
pub struct SetInternalSchoolName;
impl ApiEndpoint for SetInternalSchoolName {
    type Path = SetInternalSchoolNamePath;
    type Req = SchoolNameId;
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Patch;
}

/// Delete a user account
pub struct DeleteUserAccount;
impl ApiEndpoint for DeleteUserAccount {
    type Path = DeleteUserAccountPath;
    type Req = ();
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Delete;
}

/// Set a school name for a school
pub struct RemoveUserFromSchool;
impl ApiEndpoint for RemoveUserFromSchool {
    type Path = RemoveUserFromSchoolPath;
    type Req = UserId;
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Post;
}
