use std::{collections::HashMap, rc::Rc};

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{category, jig, meta, ApiEndpoint},
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::{JigId, JigResponse, JigData, JigUpdateDraftDataRequest, PrivacyLevel},
        meta::MetadataResponse,
    },
    error::{EmptyError, MetadataNotFound},
};
use utils::{prelude::{api_with_auth, api_with_auth_empty, UnwrapJiExt}, routes::{JigEditRoute, JigRoute, Route}};

use super::{publish_jig::PublishJig, state::State};
use super::super::state::State as JigEditState;

impl State {
    pub async fn load_new(jig_edit_state: Rc<JigEditState>) -> Self {
        let jig = load_jig(jig_edit_state.jig_id);
        let categories = load_categories();
        let meta = load_metadata();

        let (jig, categories, meta) = join!(jig, categories, meta);

        let mut jig = jig.unwrap_ji();

        let categories = categories.unwrap_ji();
        let mut category_label_lookup = HashMap::new();
        get_categories_labels(&categories, &mut category_label_lookup, "");

        let meta = meta.unwrap_ji();

        if jig.published_at.is_none() {
            set_default_values(&mut jig.jig_data, &meta);
        }

        Self::new(
            PublishJig::new(jig),
            categories,
            category_label_lookup,
            meta.goals,
            meta.age_ranges,
            meta.affiliations,
            jig_edit_state
        )

    }

    pub fn navigate_to_cover(&self) {
        let cover_module_id = self.jig.modules.lock_ref().first().map(|m| m.id.clone());

        if let Some(cover_module_id) = cover_module_id {
            self.jig_edit_state.route.set(JigEditRoute::Module(cover_module_id));
        };
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

fn set_default_values(jig: &mut JigData, meta: &MetadataResponse) {
    let available_affiliations = meta.affiliations
        .iter()
        .map(|affiliation| affiliation.id.clone())
        .collect();
    jig.affiliations = available_affiliations;

    let available_ages = meta.age_ranges
        .iter()
        .map(|age| age.id.clone())
        .collect();
    jig.age_ranges = available_ages;

    jig.privacy_level = PrivacyLevel::default()
}

async fn load_jig(jig_id: JigId) -> Result<JigResponse, EmptyError> {
    let path = jig::GetDraft::PATH.replace("{id}", &jig_id.0.to_string());

    match api_with_auth::<JigResponse, EmptyError, ()>(&path, jig::GetDraft::METHOD, None).await {
        Ok(resp) => Ok(resp),
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

async fn save_and_publish(state: Rc<State>) -> Result<(), ()> {
    let path = jig::Update::PATH.replace("{id}", &state.jig.id.0.to_string());
    let req = state.jig.to_jig_update_request();
    api_with_auth_empty::<MetadataNotFound, JigUpdateDraftDataRequest>(&path, jig::Update::METHOD, Some(req))
        .await
        .map_err(|_| ())?;

    let path = jig::Publish::PATH.replace("{id}", &state.jig.id.0.to_string());
    api_with_auth_empty::<EmptyError, ()>(&path, jig::Publish::METHOD, None)
        .await
        .map_err(|_| ())?;

    Ok(())
}

pub fn save_jig(state: Rc<State>) {
    if form_invalid(Rc::clone(&state)) {
        state.submission_tried.set(true);
        return;
    };

    state.loader.load(clone!(state => async move {
        match save_and_publish(Rc::clone(&state)).await {
            Ok(_) => {
                state.submission_tried.set(false);

                state.jig_edit_state.route.set_neq(JigEditRoute::PostPublish);

                let url: String = Route::Jig(JigRoute::Edit(state.jig.id, JigEditRoute::PostPublish)).into();
                log::info!("{}", url);

                /* this will cause a full refresh - but preserves history
                 * see the .future in EditPage too
                dominator::routing::go_to_url(&url);
                 */
            },
            Err(_) => {
                let _ = web_sys::window().unwrap().alert_with_message("Error!");
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
