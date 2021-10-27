use serde::{Deserialize, Serialize};
use shared::domain::jig::module::body::Image;
use url::Url;
use utils::unwrap::UnwrapJiExt;

use super::actions;

pub const IMAGE_SEARCH_DATA_TRANSFER: &'static str = "image-search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageDataTransfer {
    Web(Url),
    Image(Image)
}

impl ImageDataTransfer {
    pub async fn to_image(self) -> Image {
        match self {
            Self::Image(image) => image,
            Self::Web(url) => {
                actions::web_to_image(url).await.unwrap_ji()
            },
        }
    }
}
