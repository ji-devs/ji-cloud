use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::api::endpoints;
use shared::domain::additional_resource::AdditionalResource;
use shared::domain::asset::PrivacyLevel;
use shared::domain::course::unit::CourseUnit;
use shared::domain::course::{
    CourseAdminDataUpdatePath, CoursePublishPath, CourseRating, CourseUpdateAdminDataRequest,
    CourseUpdateDraftDataPath,
};
use shared::domain::{
    category::CategoryId,
    course::{CourseId, CourseResponse, CourseUpdateDraftDataRequest},
    module::LiteModule,
    UpdateNonNullable,
};

use crate::prelude::{ApiEndpointExt, ErrorExt};

#[derive(Clone)]
pub struct EditableCourse {
    pub id: CourseId,
    // cover and modules only for read
    pub cover: Mutable<Option<LiteModule>>,
    pub units: MutableVec<CourseUnit>,
    pub published_at: Mutable<Option<DateTime<Utc>>>,
    pub display_name: Mutable<String>,
    pub description: Mutable<String>,
    pub language: Mutable<String>,
    pub duration: Mutable<Option<u32>>,
    pub categories: Mutable<HashSet<CategoryId>>,
    pub additional_resources: Rc<MutableVec<AdditionalResource>>,
    pub privacy_level: Mutable<PrivacyLevel>,
    pub rating: Mutable<Option<CourseRating>>,
    pub blocked: Mutable<bool>,
    pub premium: Mutable<bool>,
}

impl From<CourseResponse> for EditableCourse {
    fn from(course: CourseResponse) -> Self {
        Self {
            id: course.id,
            cover: Mutable::new(course.course_data.cover),
            units: MutableVec::new_with_values(course.course_data.units),
            display_name: Mutable::new(course.course_data.display_name),
            description: Mutable::new(course.course_data.description),
            language: Mutable::new(course.course_data.language),
            categories: Mutable::new(HashSet::from_iter(course.course_data.categories)),
            additional_resources: Rc::new(MutableVec::new_with_values(
                course.course_data.additional_resources,
            )),
            privacy_level: Mutable::new(course.course_data.privacy_level),
            rating: Mutable::new(course.admin_data.rating),
            blocked: Mutable::new(course.admin_data.blocked),
            premium: Mutable::new(course.admin_data.premium),
            published_at: Mutable::new(course.published_at),
            duration: Mutable::new(course.course_data.duration),
        }
    }
}

impl From<CourseId> for EditableCourse {
    fn from(course_id: CourseId) -> Self {
        Self {
            id: course_id,
            cover: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            language: Default::default(),
            categories: Default::default(),
            additional_resources: Default::default(),
            privacy_level: Default::default(),
            rating: Default::default(),
            blocked: Default::default(),
            premium: Default::default(),
            published_at: Default::default(),
            units: Default::default(),
            duration: Default::default(),
        }
    }
}

impl EditableCourse {
    pub fn fill_from_course(&self, course: CourseResponse) {
        self.cover.set(course.course_data.cover);
        self.units
            .lock_mut()
            .replace_cloned(course.course_data.units);
        self.display_name.set(course.course_data.display_name);
        self.description.set(course.course_data.description.clone());
        self.language.set(course.course_data.language);
        self.categories
            .set(HashSet::from_iter(course.course_data.categories));
        self.additional_resources
            .lock_mut()
            .replace_cloned(course.course_data.additional_resources);
        self.privacy_level.set(course.course_data.privacy_level);
        self.rating.set(course.admin_data.rating);
        self.blocked.set(course.admin_data.blocked);
        self.premium.set(course.admin_data.premium);
        self.published_at.set(course.published_at);
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            id: self.id,
            cover: Mutable::new(self.cover.get_cloned()),
            units: MutableVec::new_with_values(self.units.lock_ref().to_vec()),
            published_at: Mutable::new(self.published_at.get()),
            display_name: Mutable::new(self.display_name.get_cloned()),
            description: Mutable::new(self.description.get_cloned()),
            language: Mutable::new(self.language.get_cloned()),
            categories: Mutable::new(self.categories.get_cloned()),
            additional_resources: Rc::new(MutableVec::new_with_values(
                self.additional_resources.lock_ref().to_vec(),
            )),
            privacy_level: Mutable::new(self.privacy_level.get()),
            rating: Mutable::new(self.rating.get()),
            blocked: Mutable::new(self.blocked.get()),
            premium: Mutable::new(self.premium.get()),
            duration: Mutable::new(self.duration.get()),
        }
    }

    pub fn to_course_update_request(&self) -> CourseUpdateDraftDataRequest {
        // don't include additional_resources here since they're handled in separately
        CourseUpdateDraftDataRequest {
            display_name: Some(self.display_name.get_cloned()),
            description: Some(self.description.get_cloned()),
            language: Some(self.language.get_cloned()),
            categories: Some(self.categories.get_cloned().into_iter().collect()),
            privacy_level: Some(self.privacy_level.get()),
            // not updating because it'll override the existing units, need a better solution
            // units: Some(self.units.lock_ref().to_vec()),
            ..Default::default()
        }
    }

    pub fn to_update_admin_data_request(&self) -> CourseUpdateAdminDataRequest {
        CourseUpdateAdminDataRequest {
            rating: self.rating.get_cloned().into(),
            blocked: UpdateNonNullable::Change(self.blocked.get()),
            premium: UpdateNonNullable::Change(self.premium.get()),
            ..Default::default()
        }
    }

    pub async fn save_draft(&self) -> anyhow::Result<()> {
        let req = self.to_course_update_request();
        endpoints::course::UpdateDraftData::api_with_auth(
            CourseUpdateDraftDataPath(self.id),
            Some(req),
        )
        .await
        .into_anyhow()
    }

    pub async fn save_admin_data(&self) -> anyhow::Result<()> {
        let req = self.to_update_admin_data_request();
        endpoints::course::CourseAdminDataUpdate::api_with_auth(
            CourseAdminDataUpdatePath(self.id),
            Some(req),
        )
        .await
        .into_anyhow()
    }

    pub async fn publish(&self) -> anyhow::Result<()> {
        endpoints::course::Publish::api_with_auth(CoursePublishPath(self.id), None)
            .await
            .into_anyhow()
    }
}
