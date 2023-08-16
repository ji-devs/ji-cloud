use serde::{Deserialize, Serialize};
use shared::domain::module::body::Image;
use shared::Url;
use utils::unwrap::UnwrapJiExt;

use super::actions;

pub const IMAGE_SEARCH_DATA_TRANSFER: &str = "image-search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageDataTransfer {
    Web(Url),
    Image(Image),
}

impl ImageDataTransfer {
    pub async fn to_image(self) -> Image {
        match self {
            Self::Image(image) => image,
            Self::Web(url) => actions::web_to_image(url).await.unwrap_ji(),
        }
    }
}
