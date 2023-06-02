use std::rc::Rc;

use dominator::clone;
use dominator_helpers::futures::AsyncLoader;
use futures::StreamExt;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::{
    audio::AudioId,
    course::unit::{CourseUnitId, CourseUnitValue},
    image::ImageId,
    module::body::_groups::design::YoutubeEmbed,
    pdf::PdfId,
};
use utils::editable_asset::{EditableAsset, EditableCourse};
use wasm_bindgen_futures::spawn_local;
// use futures::future::ready;

use crate::edit::AssetEditState;

pub struct UnitEditor {
    // Not having an ID means that's this is a new unit
    pub(super) unit_id: Option<CourseUnitId>,
    pub(super) asset_edit_state: Rc<AssetEditState>,
    pub(super) editable_course: Rc<EditableCourse>,
    pub(super) display_name: Mutable<String>,
    pub(super) description: Mutable<String>,
    pub(super) value: Mutable<UnitValue>,
    pub(super) url_str: Mutable<String>,
    pub(super) loader: AsyncLoader,
    pub(super) changed: Mutable<bool>,
}

impl UnitEditor {
    pub fn new(unit_id: Option<CourseUnitId>, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        let editable_course = match &*asset_edit_state.asset {
            EditableAsset::Course(course) => course,
            _ => unreachable!(),
        };

        let units = editable_course.units.lock_ref();
        let unit = units.iter().find(|x| Some(x.id) == unit_id);

        let self_ = match unit {
            Some(unit) => Rc::new(Self {
                unit_id,
                asset_edit_state: Rc::clone(&asset_edit_state),
                editable_course: Rc::clone(editable_course),
                display_name: Mutable::new(unit.display_name.clone()),
                description: Mutable::new(unit.description.clone()),
                value: Mutable::new(unit.value.clone().into()),
                url_str: Mutable::new("".to_string()),
                changed: Mutable::new(false),
                loader: AsyncLoader::new(),
            }),
            None => Rc::new(Self {
                unit_id,
                asset_edit_state: Rc::clone(&asset_edit_state),
                editable_course: Rc::clone(editable_course),
                display_name: Mutable::new("".to_string()),
                description: Mutable::new("".to_string()),
                value: Mutable::new(Default::default()),
                url_str: Mutable::new("".to_string()),
                changed: Mutable::new(false),
                loader: AsyncLoader::new(),
            }),
        };

        // having an ID but not a unit means it's not yet loaded
        if let Some(unit_id) = unit_id {
            if unit.is_none() {
                self_.fill_after_loaded(unit_id);
            }
        }

        self_
    }

    fn fill_after_loaded(self: &Rc<Self>, unit_id: CourseUnitId) {
        let state = self;
        // wait for the first sidebar spot change, which we use as a proxy to know when the Course is loaded.
        spawn_local(clone!(state => async move {
            state.asset_edit_state.sidebar_spots.signal_vec_cloned().to_stream().next().await;
            let units = state.editable_course.units.lock_ref();
            let unit = units.iter().find(|x| x.id == unit_id);

            if let Some(unit_clone) = unit {
                state.display_name.set(unit_clone.display_name.clone());
                state.description.set(unit_clone.description.clone());
                state.value.set(unit_clone.value.clone().into());
            }
        }))
    }
}

/// Value of Pro Dev Unit
#[derive(Debug, Clone)]
pub enum UnitValue {
    File(Option<UnitValueFile>),
    Link(Option<url::Url>),
    Video(Option<YoutubeEmbed>),
}

impl UnitValue {
    pub fn is_some(value: &UnitValue) -> bool {
        match value {
            UnitValue::File(file) => match file {
                Some(_) => true,
                None => false,
            },
            UnitValue::Link(url) => match url {
                Some(_) => true,
                None => false,
            },
            UnitValue::Video(video) => match video {
                Some(_) => true,
                None => false,
            },
        }
    }
}

impl Default for UnitValue {
    fn default() -> Self {
        Self::Video(None)
    }
}

#[derive(Debug, Clone)]
pub enum UnitValueFile {
    ImageId(Option<ImageId>),
    AudioId(Option<AudioId>),
    PdfId(Option<PdfId>),
}

impl From<CourseUnitValue> for UnitValue {
    fn from(value: CourseUnitValue) -> Self {
        match value {
            CourseUnitValue::ImageId(v) => Self::File(Some(UnitValueFile::ImageId(Some(v)))),
            CourseUnitValue::AudioId(v) => Self::File(Some(UnitValueFile::AudioId(Some(v)))),
            CourseUnitValue::PdfId(v) => Self::File(Some(UnitValueFile::PdfId(Some(v)))),
            CourseUnitValue::Link(v) => Self::Link(Some(v)),
            CourseUnitValue::Video(v) => Self::Video(Some(v)),
        }
    }
}

impl TryFrom<UnitValue> for CourseUnitValue {
    type Error = anyhow::Error;
    fn try_from(value: UnitValue) -> Result<Self, Self::Error> {
        match value {
            UnitValue::File(v) => match v {
                Some(v) => match v {
                    UnitValueFile::ImageId(v) => match v {
                        Some(v) => Ok(Self::ImageId(v)),
                        None => Err(anyhow::anyhow!("")),
                    },
                    UnitValueFile::AudioId(v) => match v {
                        Some(v) => Ok(Self::AudioId(v)),
                        None => Err(anyhow::anyhow!("")),
                    },
                    UnitValueFile::PdfId(v) => match v {
                        Some(v) => Ok(Self::PdfId(v)),
                        None => Err(anyhow::anyhow!("")),
                    },
                },
                None => Err(anyhow::anyhow!("")),
            },
            UnitValue::Link(v) => match v {
                Some(v) => Ok(Self::Link(v)),
                None => Err(anyhow::anyhow!("")),
            },
            UnitValue::Video(v) => match v {
                Some(v) => Ok(Self::Video(v)),
                None => Err(anyhow::anyhow!("")),
            },
        }
    }
}
