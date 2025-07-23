use anyhow;
use serde::Deserialize;
use shared::domain::search::{ImageType, WebImageSearchItem, WebImageSearchResponse};

const QUERY_TYPE: &str = "image_type";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    #[serde(rename(deserialize = "previewURL"))]
    thumbnail_url: url::Url,
    #[serde(rename(deserialize = "largeImageURL"))]
    content_url: url::Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImagesResponse {
    hits: Vec<Image>,
}

pub async fn get_images(
    query: &str,
    image_type: Option<ImageType>,
    key: &str,
) -> anyhow::Result<WebImageSearchResponse> {
    // returns tuple if there is a valid user input,
    // otherwise empty query for image type field
    let image_type: (&str, String) = if let Some(image) = image_type {
        let im_type = match image {
            ImageType::Clipart => ImageType::Clipart.to_str().to_lowercase(),
            ImageType::AnimatedGif => ImageType::AnimatedGif.to_str().to_lowercase(),
            ImageType::Photo => ImageType::Photo.to_str().to_lowercase(),
            ImageType::Line => ImageType::Line.to_str().to_lowercase(),
            ImageType::Transparent => ImageType::Transparent.to_str().to_lowercase(),
            ImageType::All => ImageType::All.to_str().to_lowercase(),
            ImageType::Illustration => ImageType::Illustration.to_str().to_lowercase(),
            ImageType::Vector => ImageType::Vector.to_str().to_lowercase(),
        };

        (QUERY_TYPE, im_type)
    } else {
        ("", "".to_string())
    };

    // https://pixabay.com/api/docs/#api_search_images
    let res = reqwest::Client::new()
        .get("https://pixabay.com/api/")
        .query(&[("q", query)])
        .query(&[image_type])
        .query(&[("safesearch", "true")])
        .query(&[("key", key)])
        .query(&[("per_page", "21")])
        .send()
        .await?
        .error_for_status()?
        .json::<ImagesResponse>()
        .await?;

    let res = WebImageSearchResponse {
        images: res
            .hits
            .into_iter()
            .map(|it| WebImageSearchItem {
                thumbnail_url: it.thumbnail_url,
                url: it.content_url,
            })
            .collect(),
    };

    Ok(res)
}
