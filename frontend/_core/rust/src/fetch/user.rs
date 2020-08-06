use shared::api::endpoints::user::*;
use shared::api::endpoints::ApiEndpoint;
use crate::{
    path::api_url,
    fetch::{POST, GET, api_with_token, api_with_auth}
};

pub async fn fetch_signin(token:&str) -> Result < <Signin as ApiEndpoint>::Res, <Signin as ApiEndpoint>::Err> {
    api_with_token::< _, _, ()>(&api_url(Signin::PATH), token, POST, None).await
}

pub async fn fetch_single_signon(token:&str) -> Result < <SingleSignOn as ApiEndpoint>::Res, <SingleSignOn as ApiEndpoint>::Err> {
    api_with_token::< _, _, ()>(&api_url(SingleSignOn::PATH), token, POST, None).await
}


pub async fn fetch_register(token:&str, req:&<Register as ApiEndpoint>::Req) -> Result < <Register as ApiEndpoint>::Res, <Register as ApiEndpoint>::Err> {
    api_with_token(&api_url(Register::PATH), token, POST, Some(req)).await
}

pub async fn fetch_profile() -> Result < <Profile as ApiEndpoint>::Res, <Profile as ApiEndpoint>::Err> {
    api_with_auth::< _, _, ()>(&api_url(Profile::PATH), POST, None).await
}
