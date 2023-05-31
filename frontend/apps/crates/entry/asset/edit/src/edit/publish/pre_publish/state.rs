use std::{collections::HashMap, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    category::{Category, CategoryId},
    meta::{Affiliation, AgeRange, ResourceType},
};
use utils::{
    editable_asset::EditableAsset,
    languages::{Language, JIG_LANGUAGES},
};

use super::super::Publish;

const STR_JIG: &str = "JIG";
const STR_RESOURCE: &str = "Resource";
const STR_PLAYLIST: &str = "Playlist";
const STR_PRO_DEV: &str = "Pro Dev course";

pub struct PrePublish {
    pub loader: AsyncLoader,
    pub asset: Rc<EditableAsset>,
    pub categories: Mutable<Vec<Category>>,
    // categories has label lookup since it's both more complex to lookup and used more then others (pills)
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub submission_tried: Mutable<bool>,
    pub show_missing_info_popup: Mutable<bool>,
    pub languages: Vec<Language>,
    pub publish_state: Rc<Publish>,
    pub show_public_popup: Mutable<bool>,
}

impl PrePublish {
    pub fn new(
        categories: Vec<Category>,
        category_label_lookup: HashMap<CategoryId, String>,
        ages: Vec<AgeRange>,
        affiliations: Vec<Affiliation>,
        resource_types: Vec<ResourceType>,
        publish_state: Rc<Publish>,
    ) -> Self {
        Self {
            // Separate asset for publish as it doesn't auto save
            asset: Rc::clone(&publish_state.asset_edit_state.asset),
            loader: AsyncLoader::new(),
            categories: Mutable::new(categories),
            category_label_lookup: Mutable::new(category_label_lookup),
            ages: Mutable::new(ages),
            affiliations: Mutable::new(affiliations),
            resource_types: Mutable::new(resource_types),
            submission_tried: Mutable::new(false),
            show_missing_info_popup: Mutable::new(false),
            languages: JIG_LANGUAGES.clone(),
            show_public_popup: Mutable::new(false),
            publish_state,
        }
    }

    /// a displayable string for the asset type
    pub fn asset_type_name(&self) -> &'static str {
        match &*self.asset {
            EditableAsset::Jig(_) => STR_JIG,
            EditableAsset::Resource(_) => STR_RESOURCE,
            EditableAsset::Playlist(_) => STR_PLAYLIST,
            EditableAsset::ProDev(_) => STR_PRO_DEV,
        }
    }
}
