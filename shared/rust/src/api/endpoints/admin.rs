use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{admin::ExportDataRequest, session::NewSessionResponse},
    error::{ApiError, EmptyError},
};

/// Impersonate another user.
pub struct Impersonate;
impl ApiEndpoint for Impersonate {
    type Req = ();
    type Res = NewSessionResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/admin/session/user/{id}";
    const METHOD: Method = Method::Post;
}

/// Export data
pub struct ExportData;
impl ApiEndpoint for ExportData {
    type Req = ExportDataRequest;
    type Res = ();
    type Err = ApiError<()>;
    const PATH: &'static str = "/v1/admin/export";
    const METHOD: Method = Method::Get;
}
