use crate::{
    api::{ApiEndpoint, Method},
    domain::image::meta::{AffiliationResponse, AgeRangeResponse, StyleResponse},
};

pub struct GetAffiliations;
impl ApiEndpoint for GetAffiliations {
    type Req = ();
    type Res = AffiliationResponse;
    type Err = ();
    const PATH: &'static str = "/v1/image/meta/affiliation";
    const METHOD: Method = Method::Get;
}

pub struct GetStyle;
impl ApiEndpoint for GetStyle {
    type Req = ();
    type Res = StyleResponse;
    type Err = ();
    const PATH: &'static str = "/v1/image/meta/style";
    const METHOD: Method = Method::Get;
}

pub struct GetAgeRange;
impl ApiEndpoint for GetAgeRange {
    type Req = ();
    type Res = AgeRangeResponse;
    type Err = ();
    const PATH: &'static str = "/v1/image/meta/age-range";
    const METHOD: Method = Method::Get;
}
