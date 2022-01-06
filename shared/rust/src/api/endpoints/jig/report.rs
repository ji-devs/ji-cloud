//! routes for the jig curation by admin

use crate::{
    api::Method,
    domain::{
        jig::report::{CreateJigReport, JigReport, ReportId},
        CreateResponse,
    },
    error::EmptyError,
};

use super::ApiEndpoint;

/// Create a Jig Report
///
/// # Authorization
///
/// * No user scope required
///
/// # Errors
///
pub struct Create;
impl ApiEndpoint for Create {
    type Req = CreateJigReport;
    type Res = CreateResponse<ReportId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/report";
    const METHOD: Method = Method::Post;
}

/// Get a Jig report
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
    type Res = JigReport;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/report/{report_id}";
    const METHOD: Method = Method::Get;
}
