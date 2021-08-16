use std::collections::HashMap;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    category::{Category, CategoryId},
    jig::JigId,
    meta::{AgeRange, Goal},
};
use utils::languages::{Language, LANGUAGES};

use super::publish_jig::PublishJig;

pub struct State {
    pub loader: AsyncLoader,
    pub categories: Mutable<Vec<Category>>,
    // categories has label lookup since it's both more complex to lookup and used more then others (pills)
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub goals: Mutable<Vec<Goal>>,
    pub ages: Mutable<Vec<AgeRange>>,
    pub jig: PublishJig,
    pub submission_tried: Mutable<bool>,
    pub languages: Vec<Language>,
}

impl State {
    pub fn new(
        jig: PublishJig, 
        categories: Vec<Category>,
        category_label_lookup: HashMap<CategoryId, String>,
        goals: Vec<Goal>,
        ages: Vec<AgeRange>,
    ) -> Self {
        Self {
            loader: AsyncLoader::new(),
            jig,
            categories: Mutable::new(categories),
            category_label_lookup: Mutable::new(category_label_lookup),
            goals: Mutable::new(goals),
            ages: Mutable::new(ages),
            submission_tried: Mutable::new(false),
            languages: LANGUAGES.clone(),
        }
    }
}
