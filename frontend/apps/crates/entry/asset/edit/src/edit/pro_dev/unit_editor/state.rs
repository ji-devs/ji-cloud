use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use shared::domain::{
    audio::AudioId,
    image::ImageId,
    pdf::PdfId,
    pro_dev::unit::{ProDevUnitId, ProDevUnitValue, Video},
};
use utils::editable_asset::{EditableAsset, EditableProDev};

use crate::edit::{pro_dev, AssetEditState};

#[derive(Clone)]
pub struct UnitEditor {
    // Not having an ID means that's this is a new unit
    pub(super) unit_id: Option<ProDevUnitId>,
    pub(super) asset_edit_state: Rc<AssetEditState>,
    pub(super) editable_pro_dev: Rc<EditableProDev>,
    pub(super) display_name: Mutable<String>,
    pub(super) description: Mutable<String>,
    pub(super) value: Mutable<UnitValue>,
    pub(super) url_str: Mutable<String>,
    pub(super) loader: AsyncLoader,
}

impl UnitEditor {
    pub fn new(unit_id: Option<ProDevUnitId>, asset_edit_state: &Rc<AssetEditState>) -> Rc<Self> {
        let editable_pro_dev = match &*asset_edit_state.asset {
            EditableAsset::ProDev(pro_dev) => pro_dev,
            _ => unreachable!(),
        };

        Rc::new(Self {
            unit_id,
            asset_edit_state: Rc::clone(&asset_edit_state),
            editable_pro_dev: Rc::clone(editable_pro_dev),
            display_name: Mutable::new("".to_string()),
            description: Mutable::new("".to_string()),
            value: Mutable::new(Default::default()),
            url_str: Mutable::new("".to_string()),
            loader: AsyncLoader::new(),
        })
    }
}

/// Value of Pro Dev Unit
#[derive(Debug, Clone)]
pub(super) enum UnitValue {
    File(Option<UnitValueFile>),
    Link(Option<url::Url>),
    Video(Option<Video>),
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
        Self::Link(None)
    }
}

#[derive(Debug, Clone)]
pub(super) enum UnitValueFile {
    ImageId(Option<ImageId>),
    AudioId(Option<AudioId>),
    PdfId(Option<PdfId>),
}

impl From<ProDevUnitValue> for UnitValue {
    fn from(value: ProDevUnitValue) -> Self {
        match value {
            ProDevUnitValue::ImageId(v) => Self::File(Some(UnitValueFile::ImageId(Some(v)))),
            ProDevUnitValue::AudioId(v) => Self::File(Some(UnitValueFile::AudioId(Some(v)))),
            ProDevUnitValue::PdfId(v) => Self::File(Some(UnitValueFile::PdfId(Some(v)))),
            ProDevUnitValue::Link(v) => Self::Link(Some(v)),
            ProDevUnitValue::Video(v) => Self::Video(Some(v)),
        }
    }
}

impl TryFrom<UnitValue> for ProDevUnitValue {
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
