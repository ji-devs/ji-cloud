use std::{collections::HashMap, rc::Rc};

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{ApiEndpoint, category, jig, meta},
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::{Jig, JigId, JigResponse, JigUpdateRequest},
        meta::MetadataResponse
    },
    error::{EmptyError, MetadataNotFound}
};
use utils::prelude::{api_with_auth, api_with_auth_empty};

use super::state::State;

pub fn load_data(state: Rc<State>, jig_id: JigId) {
    state.loader.load(clone!(state => async move {

        let jig = load_jig(jig_id);
        let categories = load_categories();
        let meta = load_metadata();

        let (jig, categories, meta) = join!(jig, categories, meta);

        let jig = jig.unwrap();
        state.jig.display_name.set(jig.display_name.clone());
        state.jig.fill_from_jig(jig);

        let categories = categories.unwrap();
        let mut category_label_lookup = HashMap::new();
        get_categories_labels(&categories, &mut category_label_lookup);
        state.categories.set(Some(categories));
        log::info!("{:?}", category_label_lookup);
        state.category_label_lookup.set(Some(category_label_lookup));

        let meta = meta.unwrap();
        state.goals.set(Some(meta.goals));
        state.ages.set(Some(meta.age_ranges));
    }));
}

fn get_categories_labels(categories: &Vec<Category>, lookup: &mut HashMap<CategoryId, String>) {
    for category in categories {
        lookup.insert(category.id.clone(), category.name.clone());
        get_categories_labels(&category.children, lookup);
    }
}

async fn load_jig(jig_id: JigId) -> Result<Jig, EmptyError> {
    let path = jig::Get::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, jig::Get::METHOD, None).await {
        Ok(resp) => {
            Ok(resp.jig)
        },
        Err(e) => Err(e),
    }
}


async fn load_categories() -> Result<Vec<Category>, EmptyError> {
    let req = GetCategoryRequest {
        ids: Vec::new(), 
        scope: Some(CategoryTreeScope::Decendants)
    };

    match api_with_auth::<CategoryResponse, EmptyError, GetCategoryRequest>(category::Get::PATH, category::Get::METHOD, Some(req)).await {
        Ok(resp) => {
            Ok(resp.categories)
        },
        Err(e) => Err(e),
    }
}

pub fn save_jig(state: Rc<State>) {
    if state.jig.display_name.lock_ref().is_empty() {
        state.submission_tried.set(true);
        return;
    };

    state.loader.load(clone!(state => async move {
        let path = jig::Update::PATH.replace("{id}", &state.jig.id.0.to_string());

        let req = state.jig.to_jig_update_request();

        match api_with_auth_empty::<MetadataNotFound, JigUpdateRequest>(&path, jig::Update::METHOD, Some(req)).await {
            Ok(_) => {
                state.submission_tried.set(false);
            },
            Err(_) => {
            }
        }
    }));
}

pub async fn load_metadata() -> Result<MetadataResponse, EmptyError> {
    match api_with_auth::<MetadataResponse, EmptyError, ()>(meta::Get::PATH, meta::Get::METHOD, None).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}
