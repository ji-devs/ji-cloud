use std::collections::HashSet;
use std::iter::FromIterator;

use futures_signals::signal::Mutable;
use shared::domain::{category::CategoryId, jig::{Jig, JigId, JigUpdateRequest}, meta::{AgeRangeId, GoalId}};

pub struct PublishJig {
    pub id: JigId,
    pub display_name: Mutable<Option<String>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub goals: Mutable<HashSet<GoalId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
}

impl From<Jig> for PublishJig {
    fn from(jig: Jig) -> Self {
        Self {
            id: jig.id,
            display_name: Mutable::new(jig.display_name),
            age_ranges: Mutable::new(HashSet::from_iter(jig.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.goals)),
            language: Mutable::new(jig.language),
            categories: Mutable::new(HashSet::from_iter(jig.categories)),
        }
    }
}


impl PublishJig {
    pub fn new_empty(jig_id: JigId) -> Self {
        Self {
            id: jig_id,
            display_name: Mutable::new(None),
            age_ranges: Mutable::new(HashSet::new()),
            goals: Mutable::new(HashSet::new()),
            language: Mutable::new(String::new()),
            categories: Mutable::new(HashSet::new()),
        }
    }

    pub fn fill_from_jig(&self, jig: Jig) {
        self.display_name.set(jig.display_name);
        self.age_ranges.set(HashSet::from_iter(jig.age_ranges));
        self.goals.set(HashSet::from_iter(jig.goals));
        self.language.set(jig.language);
        self.categories.set(HashSet::from_iter(jig.categories));
    }

    pub fn to_jig_update_request(&self) -> JigUpdateRequest {
        JigUpdateRequest {
            display_name: self.display_name.get_cloned(),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            goals: Some(self.goals.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            ..Default::default()
        }
    }
}
