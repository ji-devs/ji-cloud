use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::user::{UserFontNameRequest, UserFontResponse},
    error::EmptyError,
};

/// Create a user font.
///
/// Appends the font name to the user's list of fonts.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = UserFontNameRequest;
    type Res = UserFontResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/font";
    const METHOD: Method = Method::Post;
}

/// Get list of font names for the user.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserFontResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/font";
    const METHOD: Method = Method::Get;
}

/// Update a user font.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UserFontNameRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/font/{index}";
    const METHOD: Method = Method::Patch;
}

/// Delete a user color.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/font/{index}";
    const METHOD: Method = Method::Delete;
}
