use super::ApiEndpoint;
use crate::{api::Method, domain::auth::SigninSuccess, error::CommonError};

/// Impersonate a another user.
pub struct Impersonate;
impl ApiEndpoint for Impersonate {
    type Req = ();
    type Res = SigninSuccess;
    type Err = CommonError;
    const PATH: &'static str = "/v1/admin/user/{id}";
    const METHOD: Method = Method::Post;
}
