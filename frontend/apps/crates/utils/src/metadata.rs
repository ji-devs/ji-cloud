use std::sync::{Mutex, Arc};

use crate::callback_future::CallbackFuture;
use crate::{fetch::*, unwrap::UnwrapJiExt};
use futures::{future::Shared, FutureExt};
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
}

impl From<MetadataResponse> for Metadata {
    fn from(meta: MetadataResponse) -> Self {
        Self {
            image_styles: Arc::new(meta.image_styles),
            animation_styles: Arc::new(meta.animation_styles),
            age_ranges: Arc::new(meta.age_ranges),
            affiliations: Arc::new(meta.affiliations),
            resource_types: Arc::new(meta.resource_types),
            subjects: Arc::new(meta.subjects),
            image_tags: Arc::new(meta.image_tags),
        }
    }
}

pub async fn get_metadata() -> Arc<Metadata> {
    let mut state = STATE.lock().unwrap_ji();
    match &*state {
        State::Init => {
            let future = CallbackFuture::new(Box::new(move |resolve| {
                spawn_local(async {
                    let meta = endpoints::meta::Get::api_no_auth(GetMetadataPath(), None)
                        .await
                        .map(|meta| Arc::new(meta.into()))
                        .unwrap_ji();
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
