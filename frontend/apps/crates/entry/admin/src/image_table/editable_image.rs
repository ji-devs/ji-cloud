use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use shared::domain::image::{ImageResponse, ImageUpdatePath, ImageUpdateRequest};
use shared::domain::meta::{ImageStyleId, ImageTagIndex};
use shared::domain::Publish;
use shared::{
    api::endpoints,
    domain::{
        category::CategoryId,
        image::{ImageId, ImageMetadata, ImageSize},
        meta::AffiliationId,
        meta::AgeRangeId,
    },
};
use utils::prelude::ApiEndpointExt;

#[derive(Clone, Debug)]
pub struct EditableImage {
    pub id: ImageId,
    pub name: Mutable<String>,
    pub description: Mutable<String>,
    pub translated_description: Mutable<HashMap<String, String>>,
    pub is_premium: Mutable<bool>,
    pub size: Mutable<ImageSize>,
    pub publish_at: Mutable<Option<DateTime<Utc>>>,
    pub styles: Mutable<HashSet<ImageStyleId>>,
    pub tags: Mutable<HashSet<ImageTagIndex>>,
    pub age_ranges: Mutable<HashSet<AgeRangeId>>,
    pub affiliations: Mutable<HashSet<AffiliationId>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub created_at: Mutable<DateTime<Utc>>,
    pub updated_at: Mutable<Option<DateTime<Utc>>>,
}

impl From<ImageMetadata> for EditableImage {
    fn from(image: ImageMetadata) -> Self {
        Self {
            id: image.id,
            name: Mutable::new(image.name),
            description: Mutable::new(image.description),
            translated_description: Mutable::new(image.translated_description),
            is_premium: Mutable::new(image.is_premium),
            size: Mutable::new(image.size),
            publish_at: Mutable::new(image.publish_at),
            styles: Mutable::new(HashSet::from_iter(image.styles)),
            tags: Mutable::new(HashSet::from_iter(image.tags)),
            age_ranges: Mutable::new(HashSet::from_iter(image.age_ranges)),
            affiliations: Mutable::new(HashSet::from_iter(image.affiliations)),
            categories: Mutable::new(HashSet::from_iter(image.categories)),
            created_at: Mutable::new(image.created_at),
            updated_at: Mutable::new(image.updated_at),
        }
    }
}

impl From<ImageResponse> for EditableImage {
    fn from(image: ImageResponse) -> Self {
        image.metadata.into()
    }
}

impl EditableImage {
    pub fn to_image_update_request(&self) -> ImageUpdateRequest {
        ImageUpdateRequest {
            name: Some(self.name.get_cloned()),
            description: Some(self.description.get_cloned()),
            is_premium: Some(self.is_premium.get()),
            publish_at: Some(self.publish_at.get_cloned().map(|x| Publish::At(x))),
            styles: Some(self.styles.get_cloned().into_iter().collect()),
            age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
            affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            tags: Some(self.tags.get_cloned().into_iter().collect()),
        }
    }

    pub async fn save(&self) {
        let req = self.to_image_update_request();
        match endpoints::image::UpdateMetadata::api_with_auth_empty(
            ImageUpdatePath(self.id),
            Some(req),
        )
        .await
        {
            Ok(_) => {}
            Err(_err) => {
                log::error!("couldn't save!");
            }
        }
    }
}
