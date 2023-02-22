mod editable_course;
mod editable_jig;
mod editable_pro_dev;
mod editable_resource;

use std::{collections::HashSet, rc::Rc};

use chrono::{DateTime, Utc};
pub use editable_course::EditableCourse;
pub use editable_jig::EditableJig;
pub use editable_pro_dev::EditableProDev;
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
    ProDev(Rc<EditableProDev>),
}

impl EditableAsset {
    pub fn id(&self) -> AssetId {
        match self {
            EditableAsset::Jig(jig) => jig.id.into(),
            EditableAsset::Resource(resource) => resource.id.into(),
            EditableAsset::Course(course) => course.id.into(),
            EditableAsset::ProDev(pro_dev) => pro_dev.id.into(),
        }
    }

    pub fn cover(&self) -> &Mutable<Option<LiteModule>> {
        match self {
            EditableAsset::Jig(jig) => &jig.cover,
            EditableAsset::Resource(resource) => &resource.cover,
            EditableAsset::Course(course) => &course.cover,
            EditableAsset::ProDev(pro_dev) => &pro_dev.cover,
        }
    }

    pub fn display_name(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.display_name,
            EditableAsset::Resource(resource) => &resource.display_name,
            EditableAsset::Course(course) => &course.display_name,
            EditableAsset::ProDev(pro_dev) => &pro_dev.display_name,
        }
    }

    pub fn description(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.description,
            EditableAsset::Resource(resource) => &resource.description,
            EditableAsset::Course(course) => &course.description,
            EditableAsset::ProDev(pro_dev) => &pro_dev.description,
        }
    }

    pub fn age_ranges(&self) -> &Mutable<HashSet<AgeRangeId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.age_ranges,
            EditableAsset::Resource(resource) => &resource.age_ranges,
            EditableAsset::Course(course) => &course.age_ranges,
            EditableAsset::ProDev(_) => unimplemented!("unsupported age_range kind!"),
        }
    }

    pub fn language(&self) -> &Mutable<String> {
        match self {
            EditableAsset::Jig(jig) => &jig.language,
            EditableAsset::Resource(resource) => &resource.language,
            EditableAsset::Course(course) => &course.language,
            EditableAsset::ProDev(pro_dev) => &pro_dev.language,
        }
    }

    pub fn categories(&self) -> &Mutable<HashSet<CategoryId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.categories,
            EditableAsset::Resource(resource) => &resource.categories,
            EditableAsset::Course(course) => &course.categories,
            EditableAsset::ProDev(pro_dev) => &pro_dev.categories,
        }
    }

    pub fn affiliations(&self) -> &Mutable<HashSet<AffiliationId>> {
        match self {
            EditableAsset::Jig(jig) => &jig.affiliations,
            EditableAsset::Resource(resource) => &resource.affiliations,
            EditableAsset::Course(course) => &course.affiliations,
            EditableAsset::ProDev(_) => unimplemented!("unsupported affiliation kind!"),
        }
    }

    pub fn additional_resources(&self) -> &Rc<MutableVec<AdditionalResource>> {
        match self {
            EditableAsset::Jig(jig) => &jig.additional_resources,
            EditableAsset::Resource(resource) => &resource.additional_resources,
            EditableAsset::Course(course) => &course.additional_resources,
            EditableAsset::ProDev(pro_dev) => &pro_dev.additional_resources,
        }
    }

    pub fn privacy_level(&self) -> &Mutable<PrivacyLevel> {
        match self {
            EditableAsset::Jig(jig) => &jig.privacy_level,
            EditableAsset::Resource(resource) => &resource.privacy_level,
            EditableAsset::Course(course) => &course.privacy_level,
            EditableAsset::ProDev(pro_dev) => &pro_dev.privacy_level,
        }
    }

    pub fn published_at(&self) -> &Mutable<Option<DateTime<Utc>>> {
        match self {
            EditableAsset::Jig(jig) => &jig.published_at,
            EditableAsset::Resource(resource) => &resource.published_at,
            EditableAsset::Course(course) => &course.published_at,
            EditableAsset::ProDev(pro_dev) => &pro_dev.published_at,
        }
    }

    pub fn deep_clone(&self) -> Self {
        match self {
            EditableAsset::Jig(jig) => EditableAsset::Jig(Rc::new(jig.deep_clone())),
            EditableAsset::Resource(resource) => {
                EditableAsset::Resource(Rc::new(resource.deep_clone()))
            }
            EditableAsset::Course(course) => EditableAsset::Course(Rc::new(course.deep_clone())),
            EditableAsset::ProDev(pro_dev) => EditableAsset::ProDev(Rc::new(pro_dev.deep_clone())),
        }
    }

    pub fn fill_from_asset(&self, asset: Asset) {
        assert_eq!(self.asset_type(), asset.asset_type());
        match self {
            EditableAsset::Jig(jig) => jig.fill_from_jig(asset.unwrap_jig().clone()),
            EditableAsset::Course(course) => course.fill_from_course(asset.unwrap_course().clone()),
            EditableAsset::ProDev(pro_dev) => {
                pro_dev.fill_from_pro_dev(asset.unwrap_pro_dev().clone())
            }
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

    pub fn _is_pro_dev(&self) -> bool {
        matches!(self, Self::ProDev(_))
    }
}

impl From<Asset> for EditableAsset {
    fn from(asset: Asset) -> Self {
        match asset {
            Asset::Jig(jig) => EditableAsset::Jig(Rc::new(jig.into())),
            Asset::Course(course) => EditableAsset::Course(Rc::new(course.into())),
            Asset::ProDev(pro_dev) => EditableAsset::ProDev(Rc::new(pro_dev.into())),
            Asset::Resource(resource) => EditableAsset::Resource(Rc::new(resource.into())),
        }
    }
}

impl From<AssetId> for EditableAsset {
    fn from(asset_id: AssetId) -> Self {
        match asset_id {
            AssetId::JigId(jig_id) => EditableAsset::Jig(Rc::new(jig_id.into())),
            AssetId::CourseId(course_id) => EditableAsset::Course(Rc::new(course_id.into())),
            AssetId::ProDevId(pro_dev_id) => EditableAsset::ProDev(Rc::new(pro_dev_id.into())),
            AssetId::ResourceId(resource_id) => {
                EditableAsset::Resource(Rc::new(resource_id.into()))
            }
        }
    }
}
