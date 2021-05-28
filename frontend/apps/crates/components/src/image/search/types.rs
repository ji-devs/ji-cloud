use shared::{domain::image::ImageId, media::MediaLibrary};
use serde::{Serialize, Deserialize};

pub const IMAGE_SEARCH_DATA_TRANSFER: &'static str = "image-search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDataTransfer {
    pub id: ImageId,
    pub lib: MediaLibrary
}
