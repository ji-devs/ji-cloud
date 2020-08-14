use crate::{
    api::{ApiEndpoint, Method},
    domain::image::meta::GetResponse,
};

pub struct Get;
impl ApiEndpoint for Get {
    type Req = ();
    type Res = GetResponse;
    type Err = ();
    const PATH: &'static str = "/v1/image/metadata";
    const METHOD: Method = Method::Get;
}
