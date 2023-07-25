use super::ApiEndpoint;
use crate::domain::admin::{
    AdminSchoolNamesPath, AdminVerifySchoolNamePath, ImportSchoolNamesPath, InviteSchoolUsersPath,
    InviteSchoolUsersRequest, InviteSchoolUsersResponse, SearchSchoolNamesParams,
    SearchSchoolNamesResponse, VerifySchoolNameRequest,
};
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
pub struct SearchSchoolNames;
impl ApiEndpoint for SearchSchoolNames {
    type Path = AdminSchoolNamesPath;
    type Req = SearchSchoolNamesParams;
    type Res = SearchSchoolNamesResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a school names verification flag
pub struct VerifySchoolName;
impl ApiEndpoint for VerifySchoolName {
    type Path = AdminVerifySchoolNamePath;
    type Req = VerifySchoolNameRequest;
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
