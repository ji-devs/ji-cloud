//! routes for the resource curation by admin

use crate::{
    api::Method,
    domain::{
        resource::report::{CreateResourceReport, ReportId, ResourceReport},
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a Resource Report
///
/// # Authorization
///
/// * No user scope required
///
/// # Errors
///
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateResourceReport;
    type Res = CreateResponse<ReportId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/report";
    const METHOD: Method = Method::Post;
}

/// Get a Resource report
///
/// # Authorization
///
/// * Admin
///
/// # Errors
///
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = ResourceReport;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/resource/{id}/report/{report_id}";
    const METHOD: Method = Method::Get;
}
