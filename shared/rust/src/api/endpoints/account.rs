use super::ApiEndpoint;
use crate::api::Method;
use crate::domain::billing::{
    CreateSchoolAccountPath, CreateSchoolAccountRequest, GetSchoolAccountResponse,
    IndividualAccountPath, IndividualAccountResponse, SchoolAccountPath, SchoolId, SchoolNameValue,
    UpdateSchoolAccountRequest, UpdateSchoolNamePath,
};
use crate::error::AccountError;

/// Create a new school account
pub struct CreateSchoolAccount;
impl ApiEndpoint for CreateSchoolAccount {
    type Path = CreateSchoolAccountPath;
    type Req = CreateSchoolAccountRequest;
    type Res = SchoolId;
    type Err = AccountError;
    const METHOD: Method = Method::Post;
}

/// Get a school account
pub struct GetSchoolAccount;
impl ApiEndpoint for GetSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = ();
    type Res = GetSchoolAccountResponse;
    type Err = AccountError;
    const METHOD: Method = Method::Get;
}

/// Update a school account
pub struct UpdateSchoolAccount;
impl ApiEndpoint for UpdateSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = UpdateSchoolAccountRequest;
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Put;
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

/// Delete a school account
pub struct DeleteSchoolAccount;
impl ApiEndpoint for DeleteSchoolAccount {
    type Path = SchoolAccountPath;
    type Req = ();
    type Res = ();
    type Err = AccountError;
    const METHOD: Method = Method::Delete;
}

/// Get the account for the logged in user
pub struct GetIndividualAccount;
impl ApiEndpoint for GetIndividualAccount {
    type Path = IndividualAccountPath;
    type Req = ();
    type Res = IndividualAccountResponse;
    type Err = AccountError;
    const METHOD: Method = Method::Get;
}
