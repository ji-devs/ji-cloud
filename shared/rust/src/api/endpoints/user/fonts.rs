use super::ApiEndpoint;

use crate::{
    api::method::Method,
    domain::user::{
        UserFontCreatePath, UserFontDeletePath, UserFontGetPath, UserFontNameRequest,
        UserFontResponse, UserFontUpdatePath,
    },
    error::EmptyError,
};

/// Create a user font.
///
/// Appends the font name to the user's list of fonts.
pub struct Create;
impl ApiEndpoint for Create {
    type Req = UserFontNameRequest;
    type Res = UserFontResponse;
    type Path = UserFontCreatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Post;
}

/// Get list of font names for the user.
pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = UserFontResponse;
    type Path = UserFontGetPath;
    type Err = EmptyError;
    const METHOD: Method = Method::Get;
}

/// Update a user font.
pub struct Update;
impl ApiEndpoint for Update {
    type Req = UserFontNameRequest;
    type Res = ();
    type Path = UserFontUpdatePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Patch;
}

/// Delete a user font.
pub struct Delete;
impl ApiEndpoint for Delete {
    type Req = ();
    type Res = ();
    type Path = UserFontDeletePath;
    type Err = EmptyError;
    const METHOD: Method = Method::Delete;
}
