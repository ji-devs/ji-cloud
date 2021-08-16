use std::collections::HashSet;
use std::iter::FromIterator;

use futures_signals::signal::Mutable;
use shared::domain::Publish;
use shared::domain::jig::LiteModule;
use shared::domain::{
    category::CategoryId,
    jig::{Jig, JigId, JigUpdateRequest},
    meta::{AgeRangeId, GoalId},
};

#[derive(Clone)]
pub struct PublishJig {
    pub id: JigId,
    // modules only for read
    pub modules: Mutable<Vec<LiteModule>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub goals: Mutable<HashSet<GoalId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    // pub additional_resources: Mutable<HashSet<AdditionalResourceId>>,
    pub is_public: Mutable<bool>,
}

impl From<Jig> for PublishJig {
    fn from(jig: Jig) -> Self {
        Self {
            id: jig.id,
            modules: Mutable::new(jig.modules),
            display_name: Mutable::new(jig.display_name),
            description: Mutable::new(jig.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.goals)),
            language: Mutable::new(jig.language),
            categories: Mutable::new(HashSet::from_iter(jig.categories)),
            is_public: Mutable::new(jig.is_public),
        }
    }
}

impl PublishJig {
    pub fn new(jig: Jig) -> Self {
        Self {
            id: jig.id,
            display_name: Mutable::new(jig.display_name),
            modules: Mutable::new(jig.modules),
            description: Mutable::new(jig.description),
            age_ranges: Mutable::new(HashSet::from_iter(jig.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.goals)),
            language: Mutable::new(jig.language),
            categories: Mutable::new(HashSet::from_iter(jig.categories)),
            is_public: Mutable::new(jig.is_public),
        }
    }

    pub fn to_jig_update_request(&self) -> JigUpdateRequest {
        JigUpdateRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            goals: Some(self.goals.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            is_public: Some(self.is_public.get_cloned()),
            publish_at: Some(Some(Publish::now())),
            ..Default::default()
        }
    }
}
