use shared::{api::endpoints, domain::{image::ImageId, jig::module::body::Image, media::WebMediaUrlCreateRequest}, media::{MediaKind, MediaLibrary}};
use url::Url;
use serde::{Serialize, Deserialize};

use crate::fetch::ApiEndpointExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageOrWeb {
    Web(Url),
    Image(Image)
}

impl ImageOrWeb {
    pub async fn to_image(self) -> Image {
        match self {
            Self::Image(image) => image,
            Self::Web(url) => {
                Self::web_to_image(url).await
            },
        }
    }

    pub async fn web_to_image(url: Url) -> Image {
        let req = WebMediaUrlCreateRequest {
            url
        };

        match endpoints::media::Create::api_with_auth(Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => {
                if let MediaKind::Image(_) = res.kind {

                    let image = Image {
                        id: ImageId(res.id),
                        lib: MediaLibrary::Web,
                    };
                    image

                } else {
                    unreachable!("Only images here");
                }
            },
        }
    }
}
