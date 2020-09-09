use shared::{
    api::endpoints::{ApiEndpoint, image::meta, image::*},
    domain::image::{*, meta::{StyleId, AgeRangeId, AffiliationId}},
    error::image::*
};
use core::{
    path::api_url,
    fetch::{api_with_auth, api_with_auth_empty, FetchResult, upload_file}
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use url::Url;
use web_sys::File;

pub type Id = String;

pub async fn save(
    id:String,
    is_premium: bool,
    name: String,
    description: String,
    styles: impl IntoIterator<Item=String>,
    age_ranges: impl IntoIterator<Item=String>,
    affiliations: impl IntoIterator<Item=String>,
) -> Result<(), UpdateError>
{
    let path = UpdateMetadata::PATH.replace("{id}",&id);
    let data = UpdateRequest {
        name: Some(name),
        description: Some(description),
        is_premium: Some(is_premium),
        styles: Some(styles
                        .into_iter()
                        .map(|id| Uuid::parse_str(&id).unwrap_throw())
                        .map(|id| StyleId(id))
                        .collect()),
        age_ranges: Some(age_ranges
                        .into_iter()
                        .map(|id| Uuid::parse_str(&id).unwrap_throw())
                        .map(|id| AgeRangeId(id))
                        .collect()),
        affiliations: Some(affiliations
                        .into_iter()
                        .map(|id| Uuid::parse_str(&id).unwrap_throw())
                        .map(|id| AffiliationId(id))
                        .collect()),
        categories: None,
        publish_at: None,
    };
    let res:FetchResult<<UpdateMetadata as ApiEndpoint>::Res, <UpdateMetadata as ApiEndpoint>::Err>
        = api_with_auth_empty(&api_url(&path), UpdateMetadata::METHOD, Some(data)).await;

    res
        .map_err(|err| {
            match err {
                Ok(err) => err,
                Err(err) => UpdateError::InternalServerError(err)
            }
        })

}


#[derive(Clone)]
pub struct Init {
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub styles: Vec<(Id, String, bool)>,
    pub age_ranges: Vec<(Id, String, bool)>,
    pub affiliations: Vec<(Id, String, bool)>,
    pub categories: Vec<(Id, String, bool)>,
}

impl Init {
    pub async fn load(id:&str) -> Result<Self, ()> {
        let options = MetaOptions::load().await?;

        let image:Image = _get_image(id).await
            .map_err(|err| () )
            .map(|res| res.metadata)?;



        let styles:Vec<(Id, String, bool)> = 
            options.styles
                .into_iter()
                .map(|(id, label)| {
                    let contains = 
                        image.styles
                            .iter()
                            .map(|style| style.0.to_string())
                            .any(|x| x == id);
                    (id, label, contains)
                })
                .collect();

        let age_ranges:Vec<(Id, String, bool)> = 
            options.age_ranges
                .into_iter()
                .map(|(id, label)| {
                    let contains = 
                        image.age_ranges
                            .iter()
                            .map(|age_range| age_range.0.to_string())
                            .any(|x| x == id);
                    (id, label, contains)
                })
                .collect();

        let affiliations:Vec<(Id, String, bool)> = 
            options.affiliations
                .into_iter()
                .map(|(id, label)| {
                    let contains = 
                        image.affiliations
                            .iter()
                            .map(|affiliation| affiliation.0.to_string())
                            .any(|x| x == id);
                    (id, label, contains)
                })
                .collect();

                /* TODO - load global categories
        let categories:Vec<(Id, String, bool)> = 
            options.categories
                .into_iter()
                .map(|(id, label)| {
                    let contains = 
                        image.categories
                            .iter()
                            .map(|cat| cat.0.to_string())
                            .any(|x| x == id);
                    (id, label, contains)
                })
                .collect();
                */

        let Image {name, description, is_premium, ..} = image;

        Ok(Self {
            name,
            description,
            is_premium,
            styles,
            age_ranges,
            affiliations,
            categories: Vec::new(),
        })
    }
}

pub async fn get_image_url(id:&str) -> Result<String, ()> {
    _get_image(id).await
        .map_err(|err| {
            //log::error!("{:?}", err);
            ()
        })
        .map(|res| {
            res.url.to_string()
        })
}

async fn _get_image(id:&str) -> FetchResult < <GetOne as ApiEndpoint>::Res, <GetOne as ApiEndpoint>::Err> {

    let path = GetOne::PATH.replace("{id}",id);
    api_with_auth::<_, _, ()>(&api_url(&path), GetOne::METHOD, None).await
}

#[derive(Debug, Clone)]
pub struct MetaOptions {
    pub styles: Vec<(Id, String)>,
    pub age_ranges: Vec<(Id, String)>,
    pub affiliations: Vec<(Id, String)>,
}

impl MetaOptions {
    pub async fn load() -> Result<Self, ()> {
        _load_meta_options().await
            .map_err(|err| {
                log::error!("{:?}", err);
                ()
            })
            .map(|res| {
                Self {
                    styles: 
                        res.styles
                            .into_iter()
                            .map(|style| {
                                let label = "LABEL HERE".to_string();
                                let id = style.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    age_ranges: 
                        res.age_ranges
                            .into_iter()
                            .map(|age_range| {
                                let label = "LABEL HERE".to_string();
                                let id = age_range.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    affiliations: 
                        res.affiliations
                            .into_iter()
                            .map(|affiliation| {
                                let label = "LABEL HERE".to_string();
                                let id = affiliation.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                }
            })
    }
}

async fn _load_meta_options() -> FetchResult < <meta::Get as ApiEndpoint>::Res, <meta::Get as ApiEndpoint>::Err> {
    log::info!("{}", api_url(meta::Get::PATH));
    api_with_auth::<_, _, ()>(&api_url(meta::Get::PATH), meta::Get::METHOD, None).await
}
