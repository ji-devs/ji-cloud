use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::callback_future::CallbackFuture;
use crate::{fetch::*, unwrap::UnwrapJiExt};
use futures::join;
use futures::{future::Shared, FutureExt};
use shared::domain::category::{
    Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryPath, GetCategoryRequest,
};
use shared::{
    api::endpoints,
    domain::meta::{
        Affiliation, AgeRange, AnimationStyle, GetMetadataPath, ImageStyle, ImageTag,
        MetadataResponse, ResourceType, Subject,
    },
};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug)]
enum State {
    Init,
    // using `CallbackFuture` just to get around this being an opaque future, there are probably simpler ways to do this
    Loading(Shared<CallbackFuture<Arc<Metadata>>>),
    Ready(Arc<Metadata>),
}

static STATE: Mutex<State> = Mutex::new(State::Init);

#[derive(Clone, Debug)]
pub struct Metadata {
    pub image_styles: Arc<Vec<ImageStyle>>,
    pub animation_styles: Arc<Vec<AnimationStyle>>,
    pub age_ranges: Arc<Vec<AgeRange>>,
    pub affiliations: Arc<Vec<Affiliation>>,
    pub resource_types: Arc<Vec<ResourceType>>,
    pub subjects: Arc<Vec<Subject>>,
    pub image_tags: Arc<Vec<ImageTag>>,
    pub categories: Arc<Vec<Category>>,
    pub category_label_lookup: Arc<HashMap<CategoryId, String>>,
}

impl From<(MetadataResponse, CategoryResponse)> for Metadata {
    fn from((meta, category): (MetadataResponse, CategoryResponse)) -> Self {
        let mut category_label_lookup = HashMap::new();
        get_categories_labels(&category.categories, &mut category_label_lookup);
        Self {
            image_styles: Arc::new(meta.image_styles),
            animation_styles: Arc::new(meta.animation_styles),
            age_ranges: Arc::new(meta.age_ranges),
            affiliations: Arc::new(meta.affiliations),
            resource_types: Arc::new(meta.resource_types),
            subjects: Arc::new(meta.subjects),
            image_tags: Arc::new(meta.image_tags),
            categories: Arc::new(category.categories),
            category_label_lookup: Arc::new(category_label_lookup),
        }
    }
}

fn get_categories_labels(categories: &Vec<Category>, lookup: &mut HashMap<CategoryId, String>) {
    for category in categories {
        lookup.insert(category.id, category.name.clone());
        get_categories_labels(&category.children, lookup);
    }
}

pub async fn get_metadata() -> Arc<Metadata> {
    let mut state = STATE.lock().unwrap_ji();
    match &*state {
        State::Init => {
            let future = CallbackFuture::new(Box::new(move |resolve| {
                spawn_local(async {
                    let responses = join!(fetch_metadata(), fetch_categories());
                    let meta = Arc::new(responses.into());
                    resolve(meta);
                });
            }))
            .shared();

            *state = State::Loading(future.clone());
            // unlock state while awaiting
            drop(state);

            let meta = future.await;

            let mut state = STATE.lock().unwrap_ji();
            *state = State::Ready(meta.clone());
            drop(state);

            meta
        }
        State::Loading(future) => {
            let future = future.clone();

            // unlock state while awaiting
            drop(state);

            future.await
        }
        State::Ready(metadata) => metadata.clone(),
    }
}

async fn fetch_metadata() -> MetadataResponse {
    endpoints::meta::Get::api_no_auth(GetMetadataPath(), None)
        .await
        .unwrap_ji()
}

async fn fetch_categories() -> CategoryResponse {
    let req = GetCategoryRequest {
        ids: Vec::new(),
        scope: Some(CategoryTreeScope::Descendants),
    };

    endpoints::category::Get::api_no_auth(GetCategoryPath(), Some(req))
        .await
        .unwrap_ji()
}

pub async fn get_image_styles() -> Arc<Vec<ImageStyle>> {
    Arc::clone(&get_metadata().await.image_styles)
}

pub async fn get_animation_styles() -> Arc<Vec<AnimationStyle>> {
    Arc::clone(&get_metadata().await.animation_styles)
}

pub async fn get_age_ranges() -> Arc<Vec<AgeRange>> {
    Arc::clone(&get_metadata().await.age_ranges)
}

pub async fn get_affiliations() -> Arc<Vec<Affiliation>> {
    Arc::clone(&get_metadata().await.affiliations)
}

pub async fn get_resource_types() -> Arc<Vec<ResourceType>> {
    Arc::clone(&get_metadata().await.resource_types)
}

pub async fn get_subjects() -> Arc<Vec<Subject>> {
    Arc::clone(&get_metadata().await.subjects)
}

pub async fn get_image_tags() -> Arc<Vec<ImageTag>> {
    Arc::clone(&get_metadata().await.image_tags)
}

pub async fn get_categories() -> Arc<Vec<Category>> {
    Arc::clone(&get_metadata().await.categories)
}

pub async fn get_category_label_lookup() -> Arc<HashMap<CategoryId, String>> {
    Arc::clone(&get_metadata().await.category_label_lookup)
}
