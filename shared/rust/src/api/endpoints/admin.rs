use super::ApiEndpoint;
use crate::{api::Method, domain::auth::SigninSuccess, error::EmptyError};

/// Impersonate another user.
pub struct Impersonate;
impl ApiEndpoint for Impersonate {
    type Req = ();
    type Res = SigninSuccess;
    type Err = EmptyError;
    const PATH: &'static str = "/v1/admin/user/{id}";
    const METHOD: Method = Method::Post;
}
