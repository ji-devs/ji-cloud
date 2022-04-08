use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    rc::Rc,
};

use futures::join;
use futures_signals::signal::Mutable;
use shared::{
    api::{
        endpoints::{category, meta},
        ApiEndpoint,
    },
    domain::{
        category::{Category, CategoryId, CategoryResponse, CategoryTreeScope, GetCategoryRequest},
        jig::JigSearchQuery,
        meta::{Affiliation, AffiliationId, AgeRange, AgeRangeId, MetadataResponse, ResourceType},
        user::UserProfile,
    },
    error::EmptyError,
};
use utils::{
    languages::{Language, JIG_LANGUAGES},
    prelude::*,
};

#[derive(Debug)]
pub struct SearchSelected {
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<Option<String>>,
    pub query: Mutable<String>,
}

impl Default for SearchSelected {
    fn default() -> Self {
        Self {
            affiliations: Mutable::new(HashSet::new()),
            categories: Mutable::new(HashSet::new()),
            age_ranges: Mutable::new(HashSet::new()),
            language: Mutable::new(None),
            query: Mutable::new(String::new()),
        }
    }
}

impl SearchSelected {
    pub fn set_from_profile(&self, profile: &UserProfile) {
        let mut affiliations = self.affiliations.lock_mut();
        if !profile.affiliations.is_empty() {
            affiliations.clear();
            affiliations.extend(profile.affiliations.clone());
        }

        let mut state_age_ranges = self.age_ranges.lock_mut();
        if !profile.age_ranges.is_empty() && state_age_ranges.len() == 0 {
            state_age_ranges.clear();
            state_age_ranges.extend(profile.age_ranges.clone());
        }
    }

    pub fn from_search_request(search: JigSearchQuery) -> Self {
        Self {
            affiliations: Mutable::new(HashSet::from_iter(search.affiliations)),
            categories: Mutable::new(HashSet::from_iter(search.categories)),
            age_ranges: Mutable::new(HashSet::from_iter(search.age_ranges)),
            language: Mutable::new(None),
            query: Mutable::new(search.q),
        }
    }

    pub fn to_search_request(&self) -> JigSearchQuery {
        log::info!("{:?}", self);
        JigSearchQuery {
            q: self.query.get_cloned(),
            age_ranges: self.age_ranges.get_cloned().into_iter().collect(),
            affiliations: self.affiliations.get_cloned().into_iter().collect(),
            categories: self.categories.get_cloned().into_iter().collect(),
            page: Some(0),
            language: self.language.get_cloned(),
            ..Default::default()
        }
    }
}

pub struct SearchOptions {
    pub age_ranges: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub categories: Mutable<Vec<Category>>,
    pub category_label_lookup: Mutable<HashMap<CategoryId, String>>,
    pub languages: Rc<Vec<Language>>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            age_ranges: Mutable::new(vec![]),
            affiliations: Mutable::new(vec![]),
            resource_types: Mutable::new(vec![]),
            categories: Mutable::new(vec![]),
            category_label_lookup: Mutable::new(HashMap::new()),
            languages: Rc::new(JIG_LANGUAGES.clone()),
        }
    }
}

impl SearchOptions {
    pub async fn populate_options(&self) {
        let _ = join!(self.load_metadata(), self.load_categories());
    }

    async fn load_metadata(&self) -> Result<(), EmptyError> {
        match api_no_auth::<MetadataResponse, EmptyError, ()>(
            meta::Get::PATH,
            meta::Get::METHOD,
            None,
        )
        .await
        {
            Err(e) => Err(e),
            Ok(res) => {
                // only set values if they're not set yet from the profile
                if self.affiliations.lock_ref().is_empty() {
                    self.affiliations.set(res.affiliations);
                }
                if self.age_ranges.lock_ref().is_empty() {
                    self.age_ranges.set(res.age_ranges);
                }
                if self.resource_types.lock_ref().is_empty() {
                    self.resource_types.set(res.resource_types);
                }
                Ok(())
            }
        }
    }

    async fn load_categories(&self) -> Result<(), EmptyError> {
        let req = GetCategoryRequest {
            ids: Vec::new(),
            scope: Some(CategoryTreeScope::Descendants),
        };

        match api_no_auth::<CategoryResponse, EmptyError, GetCategoryRequest>(
            category::Get::PATH,
            category::Get::METHOD,
            Some(req),
        )
        .await
        {
            Err(e) => Err(e),
            Ok(res) => {
                let mut category_label_lookup = HashMap::new();
                Self::get_categories_labels(&res.categories, &mut category_label_lookup);
                self.category_label_lookup.set(category_label_lookup);
                if self.categories.lock_ref().is_empty() {
                    self.categories.set(res.categories);
                }
                Ok(())
            }
        }
    }

    fn get_categories_labels(categories: &Vec<Category>, lookup: &mut HashMap<CategoryId, String>) {
        for category in categories {
            lookup.insert(category.id, category.name.clone());
            Self::get_categories_labels(&category.children, lookup);
        }
    }
}
