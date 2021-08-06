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
    pub categories: Mutable<Option<Vec<Category>>>,
    // categories has label lookup since it's both more complex to lookup and used more then others (pills)
    pub category_label_lookup: Mutable<Option<HashMap<CategoryId, String>>>,
    pub goals: Mutable<Option<Vec<Goal>>>,
    pub ages: Mutable<Option<Vec<AgeRange>>>,
    pub jig: PublishJig,
    pub submission_tried: Mutable<bool>,
    pub languages: Vec<Language>,
}

impl State {
    pub fn new(jig_id: JigId) -> Self {
        Self {
            loader: AsyncLoader::new(),
            categories: Mutable::new(None),
            category_label_lookup: Mutable::new(None),
            goals: Mutable::new(None),
            ages: Mutable::new(None),
            jig: PublishJig::new_empty(jig_id),
            submission_tried: Mutable::new(false),
            languages: LANGUAGES.clone(),
        }
    }
}
