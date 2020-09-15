use crate::api::Method;
use serde::{de::DeserializeOwned, Serialize};

//  add something for path requests?
//  add something for auth required?

pub trait ApiEndpoint {
    type Req: Serialize;
    type Res: DeserializeOwned + Serialize;
    type Err: DeserializeOwned + Serialize;
    const PATH: &'static str;
    const METHOD: Method;
}

pub mod category;
pub mod image;
pub mod meta;
pub mod user;
