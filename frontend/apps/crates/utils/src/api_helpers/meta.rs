use crate::fetch::api_no_auth;
use shared::api::{endpoints, ApiEndpoint};
use shared::domain::meta::MetadataResponse;

type Id = String;

#[derive(Debug, Clone)]
pub struct MetaOptions {
    pub subjects: Vec<(Id, String)>,
    pub image_styles: Vec<(Id, String)>,
    pub age_ranges: Vec<(Id, String)>,
    pub affiliations: Vec<(Id, String)>,
}

impl MetaOptions {
    pub async fn load() -> Result<Self, ()> {
        //Probably doesn't need auth - just regular fetch from awsm_web
        let resp: Result<MetadataResponse, ()> = api_no_auth::<_, _, ()>(
            endpoints::meta::Get::PATH,
            endpoints::meta::Get::METHOD,
            None,
        )
        .await;
        resp.map_err(|_err| {
            //log::error!("{:?}", err);
        })
        .map(|res| Self {
            subjects: res
                .subjects
                .into_iter()
                .map(|subject| {
                    let label = subject.display_name;
                    let id = subject.id.0.to_string();
                    (id, label)
                })
                .collect(),
            image_styles: res
                .image_styles
                .into_iter()
                .map(|style| {
                    let label = style.display_name;
                    let id = style.id.0.to_string();
                    (id, label)
                })
                .collect(),
            age_ranges: res
                .age_ranges
                .into_iter()
                .map(|age_range| {
                    let label = age_range.display_name;
                    let id = age_range.id.0.to_string();
                    (id, label)
                })
                .collect(),
            affiliations: res
                .affiliations
                .into_iter()
                .map(|affiliation| {
                    let label = affiliation.display_name;
                    let id = affiliation.id.0.to_string();
                    (id, label)
                })
                .collect(),
        })
    }
}
