use crate::{
    api::Method,
    domain::{
        jig::{
            JigBrowseQuery, JigBrowseResponse, JigCreateRequest, JigId, JigResponse,
            JigSearchQuery, JigSearchResponse, JigUpdateRequest,
        },
        CreateResponse,
    },
    error::{EmptyError, MetadataNotFound},
};

use super::ApiEndpoint;

/// Endpoints for jig modules.
pub mod module;

/// Endpoints for JIG additional resources.
pub mod additional_resource;

/// Get a JIG by ID.
///
/// # Authorization
/// * None
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = JigResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Get;
}

/// Browse jigs.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Browse;
impl ApiEndpoint for Browse {
    type Req = JigBrowseQuery;
    type Res = JigBrowseResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/browse";
    const METHOD: Method = Method::Get;
}

/// Search for jigs.
///
/// # Authorization
/// * None
pub struct Search;
impl ApiEndpoint for Search {
    type Req = JigSearchQuery;
    type Res = JigSearchResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Get;
}

/// Create a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Create;
impl ApiEndpoint for Create {
    type Req = JigCreateRequest;
    type Res = CreateResponse<JigId>;
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig";
    const METHOD: Method = Method::Post;
}

/// Clone a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig`
pub struct Clone;
impl ApiEndpoint for Clone {
    type Req = ();
    type Res = CreateResponse<JigId>;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}/clone";
    const METHOD: Method = Method::Post;
}

/// Update a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Jigs
pub struct Update;
impl ApiEndpoint for Update {
    type Req = JigUpdateRequest;
    type Res = ();
    type Err = MetadataNotFound;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Patch;
}

/// Delete a JIG.
///
/// # Authorization
/// * One of `Admin`, `AdminJig`, or `ManageSelfJig` for owned Jigs
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/jig/{id}";
    const METHOD: Method = Method::Delete;
}
