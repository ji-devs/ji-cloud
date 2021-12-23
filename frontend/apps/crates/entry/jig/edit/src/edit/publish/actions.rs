use std::{collections::HashMap, rc::Rc};

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{category, jig, meta, ApiEndpoint},
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::{JigData, JigId, JigResponse, JigUpdateDraftDataRequest, PrivacyLevel},
        meta::MetadataResponse,
    },
    error::{EmptyError, MetadataNotFound},
};
use utils::{
    prelude::{api_with_auth, api_with_auth_empty, UnwrapJiExt},
    routes::{JigEditRoute, JigRoute, Route},
};

use super::super::state::State as JigEditState;
use super::{publish_jig::PublishJig, state::Publish};

impl Publish {
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
            meta.resource_types,
            jig_edit_state,
        )
    }

    pub fn navigate_to_cover(&self) {
        let cover_module_id = self.jig.modules.lock_ref().first().map(|m| m.id.clone());

        // navigate to cover if exists otherwise navigate to landing
        let route = match cover_module_id {
            Some(cover_module_id) => {
                JigEditRoute::Module(cover_module_id)
            },
            None => {
                JigEditRoute::Landing
            },
        };

        self.jig_edit_state
            .route
            .set(route);
    }

    fn form_invalid(self: Rc<Self>) -> bool {
        self.jig.display_name.lock_ref().is_empty()
            || self.jig.description.lock_ref().is_empty()
            || self.jig.language.lock_ref().is_empty()
            || self.jig.age_ranges.lock_ref().is_empty()
            || self.jig.goals.lock_ref().is_empty()
            || self.jig.categories.lock_ref().is_empty()
    }

    async fn save_and_publish(self: Rc<Self>) -> Result<(), ()> {
        let state = Rc::clone(&self);
        let path = jig::UpdateDraftData::PATH.replace("{id}", &state.jig.id.0.to_string());
        let req = state.jig.to_jig_update_request();
        api_with_auth_empty::<MetadataNotFound, JigUpdateDraftDataRequest>(
            &path,
            jig::UpdateDraftData::METHOD,
            Some(req),
        )
        .await
        .map_err(|_| ())?;

        let path = jig::Publish::PATH.replace("{id}", &state.jig.id.0.to_string());
        api_with_auth_empty::<EmptyError, ()>(&path, jig::Publish::METHOD, None)
            .await
            .map_err(|_| ())?;

        Ok(())
    }

    pub fn save_jig(self: Rc<Self>) {
        let state = Rc::clone(&self);
        if Rc::clone(&state).form_invalid() {
            state.submission_tried.set(true);
            state.show_missing_info_popup.set(true);
            return;
        };

        state.loader.load(clone!(state => async move {
            match Rc::clone(&state).save_and_publish().await {
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
    let available_affiliations = meta
        .affiliations
        .iter()
        .map(|affiliation| affiliation.id.clone())
        .collect();
    jig.affiliations = available_affiliations;

    let available_ages = meta.age_ranges.iter().map(|age| age.id.clone()).collect();
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
