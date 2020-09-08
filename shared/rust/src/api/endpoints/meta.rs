use crate::{
    api::{ApiEndpoint, Method},
    domain::meta::GetResponse,
};

pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = GetResponse;
    type Err = ();
    const PATH: &'static str = "/v1/metadata";
    const METHOD: Method = Method::Get;
}
