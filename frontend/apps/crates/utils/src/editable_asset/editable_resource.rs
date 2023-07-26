use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::{Mutable, Signal};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use shared::api::endpoints;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::meta::AffiliationId;
use shared::domain::resource::{
    ResourceAdminDataUpdatePath, ResourcePublishPath, ResourceRating,
    ResourceUpdateAdminDataRequest, ResourceUpdateDraftDataPath,
};
use shared::domain::{
    category::CategoryId,
    meta::AgeRangeId,
    module::LiteModule,
    resource::{ResourceId, ResourceResponse, ResourceUpdateDraftDataRequest},
    UpdateNonNullable,
};
use shared::error::IntoAnyhow;

use crate::prelude::ApiEndpointExt;

#[derive(Clone)]
pub struct EditableResource {
    pub id: ResourceId,
    pub cover: Mutable<Option<LiteModule>>,
    pub published_at: Mutable<Option<DateTime<Utc>>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub language: Mutable<String>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
    pub other_keywords: Mutable<String>,
    pub rating: Mutable<Option<ResourceRating>>,
    pub blocked: Mutable<bool>,
    pub premium: Mutable<bool>,
    pub likes: Mutable<i64>,
    pub views: Mutable<i64>,
    pub author_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<ResourceResponse> for EditableResource {
    fn from(resource: ResourceResponse) -> Self {
        Self {
            id: resource.id,
            cover: Mutable::new(resource.resource_data.cover),
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
            published_at: Mutable::new(resource.published_at),
            other_keywords: Mutable::new(resource.resource_data.other_keywords),
            rating: Mutable::new(resource.admin_data.rating),
            blocked: Mutable::new(resource.admin_data.blocked),
            premium: Mutable::new(resource.admin_data.premium),
            likes: Mutable::new(resource.likes),
            views: Mutable::new(resource.views),
            author_name: resource.author_name,
            created_at: resource.resource_data.created_at,
        }
    }
}

impl From<ResourceId> for EditableResource {
    fn from(resource_id: ResourceId) -> Self {
        Self {
            id: resource_id,
            cover: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            age_ranges: Default::default(),
            language: Default::default(),
            categories: Default::default(),
            affiliations: Default::default(),
            additional_resources: Default::default(),
            privacy_level: Default::default(),
            published_at: Default::default(),
            other_keywords: Default::default(),
            rating: Default::default(),
            blocked: Default::default(),
            premium: Default::default(),
            likes: Default::default(),
            views: Default::default(),
            author_name: Default::default(),
            created_at: Default::default(),
        }
    }
}

impl EditableResource {
    pub fn resource_signal(&self) -> impl Signal<Item = Option<AdditionalResource>> {
        self.additional_resources
            .signal_vec_cloned()
            .to_signal_map(|slice| {
                log::info!("{}", slice.len());
                // make sure theres at max 1 resource
                // assert!(slice.len() <= 1);
                slice.first().cloned()
            })
    }

    pub fn fill_from_resource(&self, resource: ResourceResponse) {
        self.cover.set(resource.resource_data.cover);
        self.display_name.set(resource.resource_data.display_name);
        self.description
            .set(resource.resource_data.description.clone());
        self.age_ranges
            .set(HashSet::from_iter(resource.resource_data.age_ranges));
        self.language.set(resource.resource_data.language);
        self.categories
            .set(HashSet::from_iter(resource.resource_data.categories));
        self.affiliations
            .set(HashSet::from_iter(resource.resource_data.affiliations));
        self.additional_resources
            .lock_mut()
            .replace_cloned(resource.resource_data.additional_resources);
        self.privacy_level.set(resource.resource_data.privacy_level);
        self.published_at.set(resource.published_at);
        self.other_keywords
            .set(resource.resource_data.other_keywords);
        self.likes.set(resource.likes);
        self.views.set(resource.views);
        self.rating.set(resource.admin_data.rating);
        self.blocked.set(resource.admin_data.blocked);
        self.premium.set(resource.admin_data.premium);
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            id: self.id,
            cover: Mutable::new(self.cover.get_cloned()),
            published_at: Mutable::new(self.published_at.get()),
            display_name: Mutable::new(self.display_name.get_cloned()),
            description: Mutable::new(self.description.get_cloned()),
            age_ranges: Mutable::new(self.age_ranges.get_cloned()),
            language: Mutable::new(self.language.get_cloned()),
            categories: Mutable::new(self.categories.get_cloned()),
            affiliations: Mutable::new(self.affiliations.get_cloned()),
            additional_resources: Rc::new(MutableVec::new_with_values(
                self.additional_resources.lock_ref().to_vec(),
            )),
            privacy_level: Mutable::new(self.privacy_level.get()),
            other_keywords: Mutable::new(self.other_keywords.get_cloned()),
            rating: Mutable::new(self.rating.get()),
            blocked: Mutable::new(self.blocked.get()),
            premium: Mutable::new(self.premium.get()),
            likes: Mutable::new(self.likes.get()),
            views: Mutable::new(self.views.get()),
            author_name: self.author_name.clone(),
            created_at: self.created_at,
        }
    }

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
            other_keywords: Some(self.other_keywords.get_cloned()),
            ..Default::default()
        }
    }

    pub fn to_update_admin_data_request(&self) -> ResourceUpdateAdminDataRequest {
        ResourceUpdateAdminDataRequest {
            rating: self.rating.get_cloned().into(),
            blocked: UpdateNonNullable::Change(self.blocked.get()),
            premium: UpdateNonNullable::Change(self.premium.get()),
            ..Default::default()
        }
    }

    pub async fn save_draft(&self) -> anyhow::Result<()> {
        let req = self.to_resource_update_request();
        endpoints::resource::UpdateDraftData::api_with_auth(
            ResourceUpdateDraftDataPath(self.id),
            Some(req),
        )
        .await
        .into_anyhow()
    }

    pub async fn save_admin_data(&self) -> anyhow::Result<()> {
        let req = self.to_update_admin_data_request();
        endpoints::resource::ResourceAdminDataUpdate::api_with_auth(
            ResourceAdminDataUpdatePath(self.id),
            Some(req),
        )
        .await
        .into_anyhow()
    }

    pub async fn publish(&self) -> anyhow::Result<()> {
        endpoints::resource::Publish::api_with_auth(ResourcePublishPath(self.id), None)
            .await
            .into_anyhow()
    }
}
