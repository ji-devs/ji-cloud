use crate::{
    api::Method,
    domain::media::{UrlCreatedResponse, WebMediaMetadataResponse, WebMediaUrlCreateRequest},
    error::EmptyError,
};

use super::ApiEndpoint;

/// Add a URL to the web media library.
/// Note: These routes match the ones from [`super::search`]
pub struct Create;
impl ApiEndpoint for Create {
    type Req = WebMediaUrlCreateRequest;
    type Res = UrlCreatedResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/media/image/url";
    const METHOD: Method = Method::Post;
}

/// Get media from the web media library by url.
pub struct GetUrl;
impl ApiEndpoint for GetUrl {
    type Req = ();
    type Res = WebMediaMetadataResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/media/url/{url}";
    const METHOD: Method = Method::Get;
}

/// Get media from the web media library by id.
pub struct GetId;
impl ApiEndpoint for GetId {
    type Req = ();
    type Res = WebMediaMetadataResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/media/id/{id}";
    const METHOD: Method = Method::Get;
}

/// Remove a URL from the web media library.
pub struct DeleteUrl;
impl ApiEndpoint for DeleteUrl {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/media/url/{url}";
    const METHOD: Method = Method::Delete;
}

/// Remove media from the web media library.
pub struct DeleteId;
impl ApiEndpoint for DeleteId {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/media/id/{id}";
    const METHOD: Method = Method::Delete;
}
