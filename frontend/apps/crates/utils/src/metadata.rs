use std::sync::Mutex;

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
    Loading(Shared<CallbackFuture<MetadataResponse>>),
    Ready(MetadataResponse),
}

static STATE: Mutex<State> = Mutex::new(State::Init);

pub async fn get_metadata() -> MetadataResponse {
    let mut state = STATE.lock().unwrap_ji();
    match &*state {
        State::Init => {
            let future = CallbackFuture::new(Box::new(move |resolve| {
                spawn_local(async {
                    let meta = endpoints::meta::Get::api_no_auth(GetMetadataPath(), None)
                        .await
                        .unwrap_ji();
                    resolve(meta);
                });
            }))
            .shared();

            *state = State::Loading(future.clone());
            // drop state while awaiting
            drop(state);

            let meta = future.await;

            let mut state = STATE.lock().unwrap_ji();
            *state = State::Ready(meta.clone());
            drop(state);

            meta
        }
        State::Loading(future) => {
            let future = future.clone();

            // drop state while awaiting
            drop(state);

            future.await
        }
        State::Ready(metadata) => metadata.clone(),
    }
}

pub async fn get_image_styles() -> Vec<ImageStyle> {
    get_metadata().await.image_styles.clone()
}

pub async fn get_animation_styles() -> Vec<AnimationStyle> {
    get_metadata().await.animation_styles.clone()
}

pub async fn get_age_ranges() -> Vec<AgeRange> {
    get_metadata().await.age_ranges.clone()
}

pub async fn get_affiliations() -> Vec<Affiliation> {
    get_metadata().await.affiliations.clone()
}

pub async fn get_resource_types() -> Vec<ResourceType> {
    get_metadata().await.resource_types.clone()
}

pub async fn get_subjects() -> Vec<Subject> {
    get_metadata().await.subjects.clone()
}

pub async fn get_image_tags() -> Vec<ImageTag> {
    get_metadata().await.image_tags.clone()
}
