use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::meta::AffiliationId;
use shared::domain::{
    category::CategoryId,
    meta::AgeRangeId,
    module::LiteModule,
    resource::{ResourceId, ResourceResponse, ResourceUpdateDraftDataRequest},
};

#[derive(Clone)]
pub struct EditableResource {
    pub id: ResourceId,
    // cover and modules only for read
    pub cover: Option<LiteModule>,
    pub published_at: Option<DateTime<Utc>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
}

impl From<ResourceResponse> for EditableResource {
    fn from(resource: ResourceResponse) -> Self {
        Self {
            id: resource.id,
            cover: resource.resource_data.cover,
            display_name: Mutable::new(resource.resource_data.display_name),
            description: Mutable::new(resource.resource_data.description.clone()),
            age_ranges: Mutable::new(HashSet::from_iter(resource.resource_data.age_ranges)),
            language: Mutable::new(resource.resource_data.language),
            categories: Mutable::new(HashSet::from_iter(resource.resource_data.categories)),
            affiliations: Mutable::new(HashSet::from_iter(resource.resource_data.affiliations)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                resource.resource_data.additional_resources,
            )),
            privacy_level: Mutable::new(resource.resource_data.privacy_level),
            published_at: resource.published_at,
        }
    }
}

impl EditableResource {
    pub fn to_resource_update_request(&self) -> ResourceUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        ResourceUpdateDraftDataRequest {
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
