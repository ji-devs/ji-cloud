use std::{collections::HashMap, rc::Rc};

use dominator::clone;
use futures::join;
use shared::{
    api::endpoints::{category, meta},
    domain::{
        asset::{Asset, PrivacyLevel},
        category::{Category, CategoryId, CategoryTreeScope, GetCategoryPath, GetCategoryRequest},
        meta::{GetMetadataPath, MetadataResponse},
    },
};
use utils::{
    prelude::{ApiEndpointExt, UnwrapJiExt},
    routes::{CourseEditRoute, JigEditRoute, PlaylistEditRoute, ResourceEditRoute},
};

use super::state::PrePublish;
use crate::edit::publish::Publish;
use utils::editable_asset::EditableAsset;

mod course_actions;
mod jig_actions;
mod playlist_actions;
mod resource_actions;

impl PrePublish {
    pub async fn load_data(publish_state: Rc<Publish>) -> Self {
        let categories = load_categories();
        let meta = load_metadata();

        let (categories, meta) = join!(categories, meta);

        let categories = categories.unwrap_ji();
        let mut category_label_lookup = HashMap::new();
        get_categories_labels(&categories, &mut category_label_lookup, "");

        let meta = meta.unwrap_ji();

        if publish_state
            .asset_edit_state
            .asset
            .published_at()
            .lock_ref()
            .is_none()
        {
            set_default_values(&publish_state.asset_edit_state.asset, &meta);
        }

        Self::new(
            categories,
            category_label_lookup,
            meta.age_ranges,
            meta.affiliations,
            meta.resource_types,
            publish_state,
        )
    }

    pub fn navigate_to_cover(&self) {
        // navigate to cover if exists otherwise navigate to landing
        let cover_module_id = self.asset.cover().lock_ref().as_ref().map(|m| m.id);

        match &*self.asset {
            EditableAsset::Jig(_) => {
                let route = match cover_module_id {
                    Some(cover_module_id) => JigEditRoute::Module(cover_module_id),
                    None => JigEditRoute::Landing,
                };

                self.publish_state.asset_edit_state.set_route_jig(route);
            }
            EditableAsset::Resource(_) => {
                self.publish_state
                    .asset_edit_state
                    .set_route_resource(ResourceEditRoute::Cover(cover_module_id.unwrap_ji()));
            }
            EditableAsset::Playlist(_) => {
                self.publish_state
                    .asset_edit_state
                    .set_route_playlist(PlaylistEditRoute::Cover(cover_module_id.unwrap_ji()));
            }
            EditableAsset::Course(_) => {
                self.publish_state
                    .asset_edit_state
                    .set_route_course(CourseEditRoute::Cover(cover_module_id.unwrap_ji()));
            }
        };
    }

    // used to show tooltip, can probably be combined with `is_ready_to_publish` somehow
    fn form_invalid(self: &Rc<Self>) -> bool {
        self.asset.display_name().lock_ref().is_empty()
        // || self.jig.description.lock_ref().is_empty()
        // || self.jig.language.lock_ref().is_empty()
        // || self.jig.age_ranges.lock_ref().is_empty()
        // || self.jig.categories.lock_ref().is_empty()
    }

    // used to disable button
    pub fn is_ready_to_publish(self: &Rc<Self>) -> bool {
        match &*self.asset {
            EditableAsset::Jig(jig) => jig.modules.lock_ref().iter().all(|m| m.is_complete),
            EditableAsset::Resource(resource) => resource.cover.lock_ref().is_some(),
            EditableAsset::Playlist(playlist) => playlist.cover.lock_ref().is_some(),
            EditableAsset::Course(course) => course.cover.lock_ref().is_some(),
        }
    }

    async fn save_async(self: &Rc<Self>) {
        match &*self.asset {
            EditableAsset::Jig(jig) => {
                jig_actions::save_jig(jig).await.unwrap_ji();
            }
            EditableAsset::Resource(resource) => {
                resource_actions::save_resource(resource).await.unwrap_ji();
            }
            EditableAsset::Playlist(playlist) => {
                playlist_actions::save_playlist(playlist).await.unwrap_ji();
            }
            EditableAsset::Course(course) => {
                course_actions::save_course(course).await.unwrap_ji();
            }
        };
    }

    pub fn save_draft(self: &Rc<Self>) {
        let state = self;
        state.loader.load(clone!(state => async move {
            state.save_async().await;
        }));
    }

    pub fn publish(self: &Rc<Self>) {
        let state = Rc::clone(&self);
        if Rc::clone(&state).form_invalid() {
            state.submission_tried.set(true);
            state.show_missing_info_popup.set(true);
            return;
        };

        state.loader.load(clone!(state => async move {
            state.save_async().await;
            let asset: Asset = match &*state.asset {
                EditableAsset::Jig(jig) => {
                    jig_actions::publish_jig(jig.id).await.unwrap_ji().into()
                },
                EditableAsset::Resource(resource) => {
                    resource_actions::publish_resource(resource.id).await.unwrap_ji().into()
                }
                EditableAsset::Playlist(playlist) => {
                    playlist_actions::publish_playlist(playlist.id).await.unwrap_ji().into()
                }
                EditableAsset::Course(course) => {
                    course_actions::publish_course(course.id).await.unwrap_ji().into()
                }
            };

            state.publish_state.asset_edit_state.asset.fill_from_asset(asset.clone());
            state.publish_state.published_asset.set(Some(asset));
            state.submission_tried.set(false);
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
        lookup.insert(category.id, name.clone());

        let base_name = name + "/";
        get_categories_labels(&category.children, lookup, &base_name);
    }
}

fn set_default_values(asset: &EditableAsset, meta: &MetadataResponse) {
    if !asset.is_course() {
        let available_affiliations = meta
            .affiliations
            .iter()
            .map(|affiliation| affiliation.id)
            .collect();
        asset.affiliations().replace(available_affiliations);

        let available_ages = meta.age_ranges.iter().map(|age| age.id).collect();
        asset.age_ranges().replace(available_ages);
    }

    asset.privacy_level().replace(PrivacyLevel::default());
}

async fn load_categories() -> anyhow::Result<Vec<Category>> {
    let req = GetCategoryRequest {
        ids: Vec::new(),
        scope: Some(CategoryTreeScope::Descendants),
    };

    match category::Get::api_with_auth(GetCategoryPath(), Some(req)).await {
        Ok(resp) => Ok(resp.categories),
        Err(e) => Err(e),
    }
}

pub async fn load_metadata() -> anyhow::Result<MetadataResponse> {
    meta::Get::api_with_auth(GetMetadataPath(), None).await
}
