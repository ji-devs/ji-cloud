use std::{collections::HashMap, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    category::{Category, CategoryId},
    meta::{Affiliation, AgeRange, ResourceType},
};
use utils::languages::{Language, JIG_LANGUAGES};

use super::super::state::State as JigEditState;
use super::publish_jig::PublishJig;

pub struct Publish {
    pub loader: AsyncLoader,
    pub categories: Mutable<Vec<Category>>,
    // categories has label lookup since it's both more complex to lookup and used more then others (pills)
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub jig: PublishJig,
    pub submission_tried: Mutable<bool>,
    pub show_missing_info_popup: Mutable<bool>,
    pub languages: Vec<Language>,
    pub jig_edit_state: Rc<JigEditState>,
    pub show_public_popup: Mutable<bool>,
}

impl Publish {
    pub fn new(
        jig: PublishJig,
        categories: Vec<Category>,
        category_label_lookup: HashMap<CategoryId, String>,
        ages: Vec<AgeRange>,
        affiliations: Vec<Affiliation>,
        resource_types: Vec<ResourceType>,
        jig_edit_state: Rc<JigEditState>,
    ) -> Self {
        Self {
            loader: AsyncLoader::new(),
            jig,
            categories: Mutable::new(categories),
            category_label_lookup: Mutable::new(category_label_lookup),
            ages: Mutable::new(ages),
            affiliations: Mutable::new(affiliations),
            resource_types: Mutable::new(resource_types),
            submission_tried: Mutable::new(false),
            show_missing_info_popup: Mutable::new(false),
            languages: JIG_LANGUAGES.clone(),
            show_public_popup: Mutable::new(false),
            jig_edit_state,
        }
    }
}
