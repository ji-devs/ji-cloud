use shared::{
    api::endpoints::{ApiEndpoint, image::*, self},
    domain::{image::*, meta::*, category::*},
    error::image::*,
};
use core::{
    path::api_url,
    fetch::{api_with_auth, api_with_auth_empty, FetchResult, upload_file}
};
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;
use url::Url;
use web_sys::File;
use crate::pages::categories::actions::load_categories;
use std::collections::{HashMap, HashSet};

pub type Id = String;

pub async fn save(
    id:String,
    is_premium: bool,
    name: String,
    description: String,
    styles: impl IntoIterator<Item=String>,
    age_ranges: impl IntoIterator<Item=String>,
    affiliations: impl IntoIterator<Item=String>,
    categories: impl IntoIterator<Item=String>,
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
        categories: Some(categories
                        .into_iter()
                        .map(|id| Uuid::parse_str(&id).unwrap_throw())
                        .map(|id| CategoryId(id))
                        .collect()),
        publish_at: None,
    };
    let res:FetchResult<<UpdateMetadata as ApiEndpoint>::Res, <UpdateMetadata as ApiEndpoint>::Err>
        = api_with_auth(&api_url(&path), UpdateMetadata::METHOD, Some(data)).await;

    res
        .map_err(|err| {
            match err {
                Ok(err) => err,
                Err(err) => UpdateError::InternalServerError(err)
            }
        })
        .map(|_| ())

}

pub async fn publish( id:String) -> Result<(), UpdateError>
{
    let path = UpdateMetadata::PATH.replace("{id}",&id);
    let data = UpdateRequest {
        name: None, 
        description: None, 
        is_premium: None, 
        styles: None,
        age_ranges: None,
        affiliations: None,
        categories: None,
        publish_at: Some(Some(Publish::now())),
    };
    let res:FetchResult<<UpdateMetadata as ApiEndpoint>::Res, <UpdateMetadata as ApiEndpoint>::Err>
        = api_with_auth(&api_url(&path), UpdateMetadata::METHOD, Some(data)).await;

    res
        .map_err(|err| {
            match err {
                Ok(err) => err,
                Err(err) => UpdateError::InternalServerError(err)
            }
        })
        .map(|_| ())

}

pub async fn delete( id:String) -> Result<(), DeleteError>
{
    let path = Delete::PATH.replace("{id}",&id);
    let res:FetchResult<<Delete as ApiEndpoint>::Res, <Delete as ApiEndpoint>::Err>
        = api_with_auth_empty::<_,()>(&api_url(&path), Delete::METHOD, None).await;

    res
        .map_err(|err| {
            match err {
                Ok(err) => err,
                Err(err) => DeleteError::InternalServerError(err)
            }
        })
        .map(|_| ())

}
pub async fn replace_url(id:&str, file:web_sys::File) -> Result<(), UpdateError>
{
    let path = UpdateMetadata::PATH.replace("{id}",&id);
    let data = UpdateRequest {
        name: None, 
        description: None, 
        is_premium: None, 
        styles: None,
        age_ranges: None,
        affiliations: None,
        categories: None,
        publish_at: None, 
    };
    let res:FetchResult<<UpdateMetadata as ApiEndpoint>::Res, <UpdateMetadata as ApiEndpoint>::Err>
        = api_with_auth(&api_url(&path), UpdateMetadata::METHOD, Some(data)).await;

    let url = res
        .map_err(|err| {
            match err {
                Ok(err) => err,
                Err(err) => UpdateError::InternalServerError(err)
            }
        })
        .map(|update_response| update_response.replace_url)?;


    upload_file(&url.to_string(), &file)
        .await
        .map_err(|err| err.into())
}

#[derive(Clone)]
pub struct Init {
    pub name: String,
    pub description: String,
    pub is_premium: bool,
    pub styles: Vec<(Id, String, bool)>,
    pub age_ranges: Vec<(Id, String, bool)>,
    pub affiliations: Vec<(Id, String, bool)>,
    pub categories: Vec<EditCategory>,
    pub selected_categories: HashSet<Id>,
}

#[derive(Clone, Debug)]
pub struct EditCategory {
    pub id: Id,
    pub name: String,
    pub assigned: bool,
    pub mode: EditCategoryMode,
    pub is_end: bool,
    pub children: Vec<EditCategory>,
    pub parent: Option<Id>
}

impl EditCategory {
    pub fn contains_leaf_set(&self, leafs:&HashSet<Id>) -> bool {
        
        if leafs.contains(&self.id) {
            true
        } else {
            for cat in self.children.iter() {
                if cat.contains_leaf_set(leafs) {
                    return true
                }
            }

            false
        }
    }
}

fn get_selected_categories(categories:&[EditCategory]) -> HashSet<Id> {
    fn push_assignments(categories: &[EditCategory], coll:&mut HashSet<Id>) {
        for cat in categories.iter() {
            if cat.assigned {
                coll.insert(cat.id.clone());
            }
            push_assignments(&cat.children, coll);
        }
    }

    let mut coll = HashSet::new();
    push_assignments(categories, &mut coll);

    coll
}

impl Init {
    pub async fn load(id:&str) -> Result<Self, ()> {
        let options = MetaOptions::load().await?;

        let image:Image = _get_image(id).await
            .map_err(|err| () )
            .map(|res| res.metadata)?;


        let categories = load_categories().await.map_err(|_| ())?.categories;

        let categories:Vec<EditCategory> =
            categories
                .into_iter()
                .map(|cat| { EditCategory::convert(cat, None, &image, EditCategoryMode::Parent) })
                .collect();

        let selected_categories = get_selected_categories(&categories);


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


        let Image {name, description, is_premium, ..} = image;

        Ok(Self {
            name,
            description,
            is_premium,
            styles,
            age_ranges,
            affiliations,
            categories,
            selected_categories
        })
    }

}

#[derive(Clone, Copy, Debug)]
pub enum EditCategoryMode {
    Parent,
    Child,
}

impl EditCategory {
    fn convert(cat:Category, parent: Option<Id>, image:&Image, mode: EditCategoryMode) -> EditCategory {

        let assigned = 
            image.categories
                .iter()
                .any(|id| *id == cat.id);

        let name = cat.name.to_string();
        let id = cat.id.0.to_string();

        let is_end = cat.children.is_empty();
        let children = 
            cat.children
                .into_iter()
                .map(|child| Self::convert(child, Some(id.clone()), image, EditCategoryMode::Child))
                .collect();

        EditCategory {
            id,
            name,
            assigned,
            children,
            mode,
            is_end,
            parent
        }
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

async fn _get_image(id:&str) -> FetchResult < <Get as ApiEndpoint>::Res, <Get as ApiEndpoint>::Err> {

    let path = Get::PATH.replace("{id}",id);
    api_with_auth::<_, _, ()>(&api_url(&path), Get::METHOD, None).await
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
                //log::error!("{:?}", err);
                ()
            })
            .map(|res| {
                Self {
                    styles: 
                        res.styles
                            .into_iter()
                            .map(|style| {
                                let label = style.display_name; 
                                let id = style.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    age_ranges: 
                        res.age_ranges
                            .into_iter()
                            .map(|age_range| {
                                let label = age_range.display_name; 
                                let id = age_range.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                    affiliations: 
                        res.affiliations
                            .into_iter()
                            .map(|affiliation| {
                                let label = affiliation.display_name; 
                                let id = affiliation.id.0.to_string();
                                (id, label)
                            })
                            .collect(),
                }
            })
    }
}

async fn _load_meta_options() -> FetchResult < <endpoints::meta::Get as ApiEndpoint>::Res, <endpoints::meta::Get as ApiEndpoint>::Err> {
    log::info!("{}", api_url(endpoints::meta::Get::PATH));
    api_with_auth::<_, _, ()>(&api_url(endpoints::meta::Get::PATH), endpoints::meta::Get::METHOD, None).await
}
