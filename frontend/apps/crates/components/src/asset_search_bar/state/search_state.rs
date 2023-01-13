use std::{collections::HashSet, iter::FromIterator};

use futures_signals::signal::Mutable;
use shared::domain::{
    category::CategoryId,
    course::CourseSearchQuery,
    jig::JigSearchQuery,
    meta::{AffiliationId, AgeRangeId},
    resource::ResourceSearchQuery,
    user::UserProfile,
};
use utils::prelude::*;

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

    pub fn from_query_params(search: SearchQueryParams) -> Self {
        Self {
            affiliations: Mutable::new(HashSet::from_iter(search.affiliations)),
            categories: Mutable::new(HashSet::from_iter(search.categories)),
            age_ranges: Mutable::new(HashSet::from_iter(search.age_ranges)),
            language: Mutable::new(None),
            query: Mutable::new(search.q),
        }
    }

    pub fn to_query_params(&self) -> SearchQueryParams {
        SearchQueryParams {
            q: self.query.get_cloned(),
            age_ranges: self.age_ranges.get_cloned().into_iter().collect(),
            affiliations: self.affiliations.get_cloned().into_iter().collect(),
            categories: self.categories.get_cloned().into_iter().collect(),
            language: self.language.get_cloned(),
        }
    }

    pub fn to_jig_search_request(&self) -> JigSearchQuery {
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

    pub fn to_resource_search_request(&self) -> ResourceSearchQuery {
        ResourceSearchQuery {
            q: self.query.get_cloned(),
            age_ranges: self.age_ranges.get_cloned().into_iter().collect(),
            affiliations: self.affiliations.get_cloned().into_iter().collect(),
            categories: self.categories.get_cloned().into_iter().collect(),
            page: Some(0),
            language: self.language.get_cloned(),
            ..Default::default()
        }
    }

    pub fn to_course_search_request(&self) -> CourseSearchQuery {
        CourseSearchQuery {
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
