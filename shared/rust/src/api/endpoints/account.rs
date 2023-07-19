use super::ApiEndpoint;
use crate::domain::billing::{
    CreateSchoolAccountPath, CreateSchoolAccountRequest, GetSchoolAccountResponse,
    IndividualAccountPath, IndividualAccountResponse, SchoolAccountPath, SchoolId, SchoolName,
    SchoolNamePath, SchoolNameValue, UpdateSchoolAccountRequest, UpdateSchoolNamePath,
};
use crate::{api::Method, error::EmptyError};

/// Return a list of known school names
pub struct GetSchoolNames;
impl ApiEndpoint for GetSchoolNames {
    type Path = SchoolNamePath;
    type Req = ();
    type Res = Vec<SchoolName>;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Create a new school account
pub struct CreateSchoolAccount;
impl ApiEndpoint for CreateSchoolAccount {
    type Path = CreateSchoolAccountPath;
    type Req = CreateSchoolAccountRequest;
    type Res = SchoolId;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get a school account
pub struct GetSchoolAccount;
impl ApiEndpoint for GetSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = ();
    type Res = GetSchoolAccountResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a school account
pub struct UpdateSchoolAccount;
impl ApiEndpoint for UpdateSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = UpdateSchoolAccountRequest;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Put;
}

/// Update a school name
pub struct UpdateSchoolName;
impl ApiEndpoint for UpdateSchoolName {
    type Path = UpdateSchoolNamePath;
    type Req = SchoolNameValue;
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete a school account
pub struct DeleteSchoolAccount;
impl ApiEndpoint for DeleteSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}

/// Get the account for the logged in user
pub struct GetIndividualAccount;
impl ApiEndpoint for GetIndividualAccount {
    type Path = IndividualAccountPath;
    type Req = ();
    type Res = IndividualAccountResponse;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
