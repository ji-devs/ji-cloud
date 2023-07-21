use std::{collections::HashSet, iter::FromIterator};

use futures_signals::signal::Mutable;
use shared::domain::{
    category::CategoryId,
    course::CourseSearchQuery,
    jig::JigSearchQuery,
    meta::{AffiliationId, AgeRangeId},
    playlist::PlaylistSearchQuery,
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
    pub rated_only: Mutable<bool>,
}

impl Default for SearchSelected {
    fn default() -> Self {
        Self {
            affiliations: Mutable::new(HashSet::new()),
            categories: Mutable::new(HashSet::new()),
            age_ranges: Mutable::new(HashSet::new()),
            language: Mutable::new(None),
            query: Mutable::new(String::new()),
            rated_only: Mutable::new(true),
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
        let s = Self {
            affiliations: Mutable::new(HashSet::from_iter(search.affiliations)),
            categories: Mutable::new(HashSet::from_iter(search.categories)),
            age_ranges: Mutable::new(HashSet::from_iter(search.age_ranges)),
            language: Mutable::new(None),
            // default is true, so None => true
            rated_only: Mutable::new(search.is_rated.unwrap_or(true)),
            query: Mutable::new(search.q),
        };

        s
    }

    pub fn to_query_params(&self) -> SearchQueryParams {
        SearchQueryParams {
            q: self.query.get_cloned(),
            // since default is true, only add when false
            is_rated: match self.rated_only.get() {
                true => None,
                false => Some(false),
            },
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
            is_rated: self.rated_only.get().then(|| true),
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
            // is_rated: self.is_rated.get().then(|| true),
            ..Default::default()
        }
    }

    pub fn to_playlist_search_request(&self) -> PlaylistSearchQuery {
        PlaylistSearchQuery {
            q: self.query.get_cloned(),
            age_ranges: self.age_ranges.get_cloned().into_iter().collect(),
            affiliations: self.affiliations.get_cloned().into_iter().collect(),
            categories: self.categories.get_cloned().into_iter().collect(),
            page: Some(0),
            language: self.language.get_cloned(),
            // is_rated: self.is_rated.get().then(|| true),
            ..Default::default()
        }
    }

    pub fn to_course_search_request(&self) -> CourseSearchQuery {
        CourseSearchQuery {
            q: self.query.get_cloned(),
            categories: self.categories.get_cloned().into_iter().collect(),
            page: Some(0),
            language: self.language.get_cloned(),
            ..Default::default()
        }
    }
}
