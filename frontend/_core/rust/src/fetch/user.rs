use shared::api::endpoints::user::*;
use shared::api::endpoints::ApiEndpoint;
use crate::{
    path::api_url,
    fetch::{api_with_auth_unwrap, api_with_token_unwrap}
};

pub async fn fetch_signin(token:&str) -> Result < <Signin as ApiEndpoint>::Res, <Signin as ApiEndpoint>::Err> {
    api_with_token_unwrap::< _, _, ()>(&api_url("user/signin"), token, None).await
}


pub async fn fetch_single_signon(token:&str) -> Result < <SingleSignOn as ApiEndpoint>::Res, <SingleSignOn as ApiEndpoint>::Err> {
    api_with_token_unwrap::< _, _, ()>(&api_url("user/single-sign-on"), token, None).await
}


pub async fn fetch_register(token:&str, req:&<Register as ApiEndpoint>::Req) -> Result < <Register as ApiEndpoint>::Res, <Register as ApiEndpoint>::Err> {
    api_with_token_unwrap(&api_url("user/register"), token, Some(req)).await
}

pub async fn fetch_profile() -> Result < <Profile as ApiEndpoint>::Res, <Profile as ApiEndpoint>::Err> {
    api_with_auth_unwrap::< _, _, ()>(&api_url("user/profile"), None).await
}
