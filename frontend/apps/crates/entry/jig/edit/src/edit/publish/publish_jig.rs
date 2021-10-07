use std::collections::HashSet;
use std::iter::FromIterator;

use futures_signals::signal::Mutable;
use shared::domain::jig::PrivacyLevel;
use shared::domain::meta::AffiliationId;
use shared::domain::{
    category::CategoryId,
    jig::{JigResponse, JigId, JigUpdateDraftDataRequest, LiteModule},
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
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    // pub additional_resources: Mutable<HashSet<AdditionalResourceId>>,
    pub privacy_level: Mutable<PrivacyLevel>,
}

impl From<JigResponse> for PublishJig {
    fn from(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            modules: Mutable::new(jig.jig_data.modules),
            display_name: Mutable::new(jig.jig_data.display_name),
            description: Mutable::new(jig.jig_data.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.jig_data.goals)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
        }
    }
}

impl PublishJig {
    pub fn new(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            display_name: Mutable::new(jig.jig_data.display_name),
            modules: Mutable::new(jig.jig_data.modules),
            description: Mutable::new(jig.jig_data.description),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.jig_data.goals)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
        }
    }

    pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
        JigUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            goals: Some(self.goals.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            ..Default::default()
        }
    }
}
