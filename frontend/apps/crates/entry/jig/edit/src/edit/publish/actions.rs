use std::{collections::HashMap, rc::Rc};

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{category, jig, meta, ApiEndpoint},
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::{Jig, JigId, JigResponse, JigUpdateRequest},
        meta::MetadataResponse,
    },
    error::{EmptyError, MetadataNotFound},
};
use utils::prelude::{api_with_auth, api_with_auth_empty, UnwrapJiExt};

use super::{publish_jig::PublishJig, state::State};

impl State {
    pub async fn load_new(jig_id:JigId) -> Self {
        let jig = load_jig(jig_id);
        let categories = load_categories();
        let meta = load_metadata();

        let (jig, categories, meta) = join!(jig, categories, meta);

        let categories = categories.unwrap_ji();
        let mut category_label_lookup = HashMap::new();
        get_categories_labels(&categories, &mut category_label_lookup, "");
        
        let meta = meta.unwrap_ji();

        Self::new(
            PublishJig::new(jig.unwrap_ji()),
            categories,
            category_label_lookup,
            meta.goals,
            meta.age_ranges
        )
    }
}


fn get_categories_labels(
    categories: &Vec<Category>,
    lookup: &mut HashMap<CategoryId, String>,
    base_name: &str,
) {
    for category in categories {
        let name = format!("{}{}", base_name, category.name);
        lookup.insert(category.id.clone(), name.clone());

        let base_name = name + "/";
        get_categories_labels(&category.children, lookup, &base_name);
    }
}

async fn load_jig(jig_id: JigId) -> Result<Jig, EmptyError> {
    let path = jig::Get::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, jig::Get::METHOD, None).await {
        Ok(resp) => Ok(resp.jig),
        Err(e) => Err(e),
    }
}

async fn load_categories() -> Result<Vec<Category>, EmptyError> {
    let req = GetCategoryRequest {
        ids: Vec::new(),
        scope: Some(CategoryTreeScope::Decendants),
    };

    match api_with_auth::<CategoryResponse, EmptyError, GetCategoryRequest>(
        category::Get::PATH,
        category::Get::METHOD,
        Some(req),
    )
    .await
    {
        Ok(resp) => Ok(resp.categories),
        Err(e) => Err(e),
    }
}

fn form_invalid(state: Rc<State>) -> bool {
    state.jig.display_name.lock_ref().is_empty()
        || state.jig.description.lock_ref().is_empty()
        || state.jig.language.lock_ref().is_empty()
        || state.jig.age_ranges.lock_ref().is_empty()
        || state.jig.goals.lock_ref().is_empty()
        || state.jig.categories.lock_ref().is_empty()
}

pub fn save_jig(state: Rc<State>) {
    if form_invalid(Rc::clone(&state)) {
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
    match api_with_auth::<MetadataResponse, EmptyError, ()>(
        meta::Get::PATH,
        meta::Get::METHOD,
        None,
    )
    .await
    {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}
