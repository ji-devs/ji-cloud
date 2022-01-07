use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::{
    meta::AffiliationId,
    category::CategoryId,
    jig::{
        additional_resource::AdditionalResource,
        JigFocus, PrivacyLevel, JigRating, JigId, JigResponse, JigUpdateDraftDataRequest,
        JigUpdateAdminDataRequest, LiteModule
    },
    meta::{AgeRangeId, GoalId},
};

#[derive(Clone)]
pub struct EditableJig {
    pub id: JigId,
    // modules only for read
    pub modules: Mutable<Vec<LiteModule>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub other_keywords: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub goals: Mutable<HashSet<GoalId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
    pub rating: Mutable<Option<JigRating>>,
    pub blocked: Mutable<bool>,
    pub jig_focus: JigFocus,
    pub author_name: String,
}

impl From<JigResponse> for EditableJig {
    fn from(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            modules: Mutable::new(jig.jig_data.modules),
            display_name: Mutable::new(jig.jig_data.display_name),
            description: Mutable::new(jig.jig_data.description.clone()),
            other_keywords: Mutable::new(jig.jig_data.other_keywords.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            goals: Mutable::new(HashSet::from_iter(jig.jig_data.goals)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(jig.jig_data.additional_resources)),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
            rating: Mutable::new(jig.admin_data.rating),
            blocked: Mutable::new(jig.admin_data.blocked),
            jig_focus: jig.jig_focus,
            author_name: jig.author_name.unwrap_or_default(),
        }
    }
}

impl EditableJig {
    pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        JigUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            other_keywords: Some(self.other_keywords.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            goals: Some(self.goals.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            ..Default::default()
        }
    }

    pub fn to_update_admin_data_request(&self) -> JigUpdateAdminDataRequest {
        JigUpdateAdminDataRequest {
            rating: self.rating.get_cloned(),
            blocked: Some(self.blocked.get()),
            ..Default::default()
        }
    }
}
