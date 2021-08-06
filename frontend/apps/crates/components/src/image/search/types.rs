use serde::{Deserialize, Serialize};
use shared::domain::jig::module::body::Image;

pub const IMAGE_SEARCH_DATA_TRANSFER: &'static str = "image-search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDataTransfer {
    pub image: Image,
}
