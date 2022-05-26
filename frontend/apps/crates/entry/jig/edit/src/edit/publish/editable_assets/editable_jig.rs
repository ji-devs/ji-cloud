use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::jig::JigFocus;
use shared::domain::meta::AffiliationId;
use shared::domain::{
    category::CategoryId,
    jig::{JigId, JigResponse, JigUpdateDraftDataRequest},
    module::LiteModule,
    meta::AgeRangeId,
};

#[derive(Clone)]
pub struct EditableJig {
    pub id: JigId,
    // cover and modules only for read
    pub cover: Option<LiteModule>,
    pub modules: Vec<LiteModule>,
    pub published_at: Option<DateTime<Utc>>,
    pub jig_focus: JigFocus,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
}

impl From<JigResponse> for EditableJig {
    fn from(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            cover: jig.jig_data.modules.first().cloned(),
            modules: jig.jig_data.modules,
            display_name: Mutable::new(jig.jig_data.display_name),
            description: Mutable::new(jig.jig_data.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                jig.jig_data.additional_resources,
            )),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
            published_at: jig.published_at,
            jig_focus: jig.jig_focus,
        }
    }
}

impl EditableJig {
    pub fn new(jig: JigResponse) -> Self {
        Self {
            id: jig.id,
            display_name: Mutable::new(jig.jig_data.display_name),
            cover: jig.jig_data.modules.first().cloned(),
            modules: jig.jig_data.modules,
            description: Mutable::new(jig.jig_data.description),
            age_ranges: Mutable::new(HashSet::from_iter(jig.jig_data.age_ranges)),
            language: Mutable::new(jig.jig_data.language),
            categories: Mutable::new(HashSet::from_iter(jig.jig_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(jig.jig_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                jig.jig_data.additional_resources,
            )),
            privacy_level: Mutable::new(jig.jig_data.privacy_level),
            published_at: jig.published_at,
            jig_focus: jig.jig_focus,
        }
    }

    pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        JigUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            ..Default::default()
        }
    }
}
