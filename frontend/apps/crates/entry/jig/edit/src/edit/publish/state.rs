use std::{collections::HashMap, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    category::{Category, CategoryId},
    meta::{Affiliation, AgeRange, Goal},
};
use utils::languages::{Language, JIG_LANGUAGES};

use super::super::state::State as JigEditState;
use super::publish_jig::PublishJig;

pub struct State {
    pub loader: AsyncLoader,
    pub categories: Mutable<Vec<Category>>,
    // categories has label lookup since it's both more complex to lookup and used more then others (pills)
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub goals: Mutable<Vec<Goal>>,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub jig: PublishJig,
    pub submission_tried: Mutable<bool>,
    pub show_missing_info_popup: Mutable<bool>,
    pub languages: Vec<Language>,
    pub jig_edit_state: Rc<JigEditState>,
    pub show_public_popup: Mutable<bool>,
}

impl State {
    pub fn new(
        jig: PublishJig,
        categories: Vec<Category>,
        category_label_lookup: HashMap<CategoryId, String>,
        goals: Vec<Goal>,
        ages: Vec<AgeRange>,
        affiliations: Vec<Affiliation>,
        jig_edit_state: Rc<JigEditState>,
    ) -> Self {
        Self {
            loader: AsyncLoader::new(),
            jig,
            categories: Mutable::new(categories),
            category_label_lookup: Mutable::new(category_label_lookup),
            goals: Mutable::new(goals),
            ages: Mutable::new(ages),
            affiliations: Mutable::new(affiliations),
            submission_tried: Mutable::new(false),
            show_missing_info_popup: Mutable::new(false),
            languages: JIG_LANGUAGES.clone(),
            show_public_popup: Mutable::new(false),
            jig_edit_state,
        }
    }
}
