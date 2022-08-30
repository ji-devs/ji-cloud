use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::user::{
        UserColorCreatePath, UserColorDeletePath, UserColorGetPath, UserColorResponse,
        UserColorUpdatePath, UserColorValueRequest,
    },
    error::EmptyError,
};

/// Create a user color.
///
/// Appends the color to the user's list of colors.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = UserColorValueRequest;
    type Res = UserColorResponse;
    type Path = UserColorCreatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get colors for the user.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserColorResponse;
    type Path = UserColorGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a user color.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UserColorValueRequest;
    type Res = ();
    type Path = UserColorUpdatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete a user color.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = UserColorDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
