mod editable_course;
mod editable_jig;
mod editable_resource;

use std::{collections::HashSet, rc::Rc};

use chrono::{DateTime, Utc};
pub use editable_course::EditableCourse;
pub use editable_jig::EditableJig;
pub use editable_resource::EditableResource;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    additional_resource::AdditionalResource,
    asset::{Asset, AssetId, AssetType, PrivacyLevel},
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId},
    module::LiteModule,
};

pub enum EditableAsset {
    Jig(Rc<EditableJig>),
    Resource(Rc<EditableResource>),
    Course(Rc<EditableCourse>),
}

impl EditableAsset {
    pub fn id(&self) -> AssetId {
        match self {
            EditableAsset::Jig(jig) => jig.id.into(),
            EditableAsset::Resource(resource) => resource.id.into(),
            EditableAsset::Course(course) => course.id.into(),
        }
    }

    pub fn cover(&self) -> &Mutable<Option<LiteModule>> {
        match self {
            EditableAsset::Jig(jig) => &jig.cover,
            EditableAsset::Resource(resource) => &resource.cover,
            EditableAsset::Course(course) => &course.cover,
        }
    }

    pub fn display_name(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.display_name,
            EditableAsset::Resource(resource) => &resource.display_name,
            EditableAsset::Course(course) => &course.display_name,
        }
    }

    pub fn description(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.description,
            EditableAsset::Resource(resource) => &resource.description,
            EditableAsset::Course(course) => &course.description,
        }
    }

    pub fn age_ranges(&self) -> &Mutable<HashSet<AgeRangeId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.age_ranges,
            EditableAsset::Resource(resource) => &resource.age_ranges,
            EditableAsset::Course(course) => &course.age_ranges,
        }
    }

    pub fn language(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.language,
            EditableAsset::Resource(resource) => &resource.language,
            EditableAsset::Course(course) => &course.language,
        }
    }

    pub fn categories(&self) -> &Mutable<HashSet<CategoryId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.categories,
            EditableAsset::Resource(resource) => &resource.categories,
            EditableAsset::Course(course) => &course.categories,
        }
    }

    pub fn affiliations(&self) -> &Mutable<HashSet<AffiliationId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.affiliations,
            EditableAsset::Resource(resource) => &resource.affiliations,
            EditableAsset::Course(course) => &course.affiliations,
        }
    }

    pub fn additional_resources(&self) -> &Rc<MutableVec<AdditionalResource>> {
        match self {
            EditableAsset::Jig(jig) => &jig.additional_resources,
            EditableAsset::Resource(resource) => &resource.additional_resources,
            EditableAsset::Course(course) => &course.additional_resources,
        }
    }

    pub fn privacy_level(&self) -> &Mutable<PrivacyLevel> {
        match self {
            EditableAsset::Jig(jig) => &jig.privacy_level,
            EditableAsset::Resource(resource) => &resource.privacy_level,
            EditableAsset::Course(course) => &course.privacy_level,
        }
    }

    pub fn published_at(&self) -> &Mutable<Option<DateTime<Utc>>> {
        match self {
            EditableAsset::Jig(jig) => &jig.published_at,
            EditableAsset::Resource(resource) => &resource.published_at,
            EditableAsset::Course(course) => &course.published_at,
        }
    }

    pub fn fill_from_asset(&self, asset: Asset) {
        assert_eq!(self.asset_type(), asset.asset_type());
        match self {
            EditableAsset::Jig(jig) => jig.fill_from_jig(asset.unwrap_jig().clone()),
            EditableAsset::Course(course) => course.fill_from_course(asset.unwrap_course().clone()),
            EditableAsset::Resource(resource) => {
                resource.fill_from_resource(asset.unwrap_resource().clone())
            }
        }
    }

    pub fn asset_type(&self) -> AssetType {
        (&self.id()).into()
    }

    pub fn _is_jig(&self) -> bool {
        matches!(self, Self::Jig(_))
    }

    pub fn is_resource(&self) -> bool {
        matches!(self, Self::Resource(_))
    }

    pub fn _is_course(&self) -> bool {
        matches!(self, Self::Course(_))
    }
}

impl From<Asset> for EditableAsset {
    fn from(asset: Asset) -> Self {
        match asset {
            Asset::Jig(jig) => EditableAsset::Jig(Rc::new(jig.into())),
            Asset::Course(course) => EditableAsset::Course(Rc::new(course.into())),
            Asset::Resource(resource) => EditableAsset::Resource(Rc::new(resource.into())),
        }
    }
}

impl From<AssetId> for EditableAsset {
    fn from(asset_id: AssetId) -> Self {
        match asset_id {
            AssetId::JigId(jig_id) => EditableAsset::Jig(Rc::new(jig_id.into())),
            AssetId::CourseId(course_id) => EditableAsset::Course(Rc::new(course_id.into())),
            AssetId::ResourceId(resource_id) => {
                EditableAsset::Resource(Rc::new(resource_id.into()))
            }
        }
    }
}
