use super::ApiEndpoint;
use crate::domain::admin::{
    ImportSchoolNamesPath, ListSchoolNamesRequest, ListSchoolNamesResponse,
    SchoolNameVerificationPath, VerifySchoolNameRequest,
};
use crate::{
    api::Method,
    domain::{
        admin::{ExportDataPath, ExportDataRequest},
        billing::{CreateUpdateSubscriptionPlanRequest, SubscriptionPlanPath},
        session::{ImpersonatePath, NewSessionResponse},
    },
    error::{ApiError, EmptyError},
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
    type Err = ApiError<()>;
    const METHOD: Method = Method::Get;
}

/// Create or update a subscription plan
pub struct CreateUpdateSubscriptionPlan;
impl ApiEndpoint for CreateUpdateSubscriptionPlan {
    type Path = SubscriptionPlanPath;
    type Req = CreateUpdateSubscriptionPlanRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// List school names
pub struct ListSchoolNames;
impl ApiEndpoint for ListSchoolNames {
    type Path = SchoolNameVerificationPath;
    type Req = ListSchoolNamesRequest;
    type Res = ListSchoolNamesResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a school names verification flag
pub struct VerifySchoolName;
impl ApiEndpoint for VerifySchoolName {
    type Path = SchoolNameVerificationPath;
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
