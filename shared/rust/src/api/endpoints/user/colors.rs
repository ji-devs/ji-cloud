use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::user::{UserColorResponse, UserColorValueRequest},
    error::EmptyError,
};

/// Create a user color.
///
/// Appends the color to the user's list of colors.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = UserColorValueRequest;
    type Res = UserColorResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/color";
    const METHOD: Method = Method::Post;
}

/// Get colors for the user.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserColorResponse;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/color";
    const METHOD: Method = Method::Get;
}

/// Update a user color.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UserColorValueRequest;
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/color/{index}";
    const METHOD: Method = Method::Patch;
}

/// Delete a user color.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Err = EmptyError;
    const PATH: &'static str = "/v1/user/me/color/{index}";
    const METHOD: Method = Method::Delete;
}
