use super::ApiEndpoint;
use crate::{
    api::Method,
    domain::{
        admin::{AdminListMediaResponse, ExportDataRequest},
        session::NewSessionResponse,
    },
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

/// Forcefully refresh an item of media (as if it was just uploaded)
/// Note: this request can be conditional on `If-Match`
/// NOTE: This route is super unstable (v0), and may change at any time, for any reason, in any way, including removal.
pub struct RefreshFiles;
impl ApiEndpoint for RefreshFiles {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v0/admin/media/refresh/{library}/image/{id}";
    const METHOD: Method = Method::Post;
}

/// List all media
/// Note that this media is *not* sorted in any particular way.
/// NOTE: This route is super unstable (v0), and may change at any time, for any reason, in any way, including removal.
pub struct ListMedia;
impl ApiEndpoint for ListMedia {
    type Req = ();
    type Res = AdminListMediaResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v0/admin/media";
    const METHOD: Method = Method::Get;
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
