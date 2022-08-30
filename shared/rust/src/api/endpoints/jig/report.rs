//! routes for the jig curation by admin

use crate::{
    api::{ApiEndpoint, Method},
    domain::{
        jig::report::{
            CreateJigReport, CreateJigReportPath, GetJigReportPath, JigReport, ReportId,
        },
        CreateResponse,
    },
    error::EmptyError,
};

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
    type Path = CreateJigReportPath;
    type Req = CreateJigReport;
    type Res = CreateResponse<ReportId>;
    type Err = EmptyError;
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
    type Path = GetJigReportPath;
    type Req = ();
    type Res = JigReport;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}
