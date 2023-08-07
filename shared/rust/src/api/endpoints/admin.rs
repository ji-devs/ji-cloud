use super::ApiEndpoint;
use crate::domain::admin::{
    AdminSchoolAccountPath, AdminSchoolsPath, AdminVerifySchoolPath, GetAdminSchoolAccountResponse,
    ImportSchoolNamesPath, InviteSchoolUsersPath, InviteSchoolUsersRequest,
    InviteSchoolUsersResponse, SchoolNamesPath, SearchSchoolsParams, SearchSchoolsResponse,
    SetInternalSchoolNamePath, UpdateSchoolNamePath, VerifySchoolRequest,
};
use crate::domain::billing::{SchoolName, SchoolNameId, SchoolNameValue};
use crate::error::AccountError;
use crate::{
    api::Method,
    domain::{
        admin::{ExportDataPath, ExportDataRequest},
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

/// Export data
pub struct ExportData;
impl ApiEndpoint for ExportData {
    type Path = ExportDataPath;
    type Req = ExportDataRequest;
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
