use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{
        admin::{ExportDataPath, ExportDataRequest},
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
