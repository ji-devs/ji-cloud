use anyhow;
use serde::Deserialize;
use shared::domain::search::{ImageType, WebImageSearchItem, WebImageSearchResponse};

const QUERY_TYPE: &str = "imageType";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    thumbnail_url: url::Url,
    content_url: url::Url,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImagesResponse {
    read_link: String,
    current_offset: i64,
    next_offset: i64,
    value: Vec<Image>,
}

pub async fn get_images(
    query: &str,
    image_type: Option<ImageType>,
    key: &str,
) -> anyhow::Result<WebImageSearchResponse> {
    // returns tuple if there is a valid user input,
    // otherwise empty query for image type field
    let image_type: (&str, &str) = if let Some(image) = image_type {
        let im_type = match image {
            ImageType::Clipart => ImageType::Clipart.to_str(),
            ImageType::AnimatedGif => ImageType::AnimatedGif.to_str(),
            ImageType::Photo => ImageType::Photo.to_str(),
            ImageType::Line => ImageType::Line.to_str(),
            ImageType::Transparent => ImageType::Transparent.to_str(),
        };

        (QUERY_TYPE, im_type)
    } else {
        ("", "")
    };

    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/endpoints
    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/query-parameters
    let res = reqwest::Client::new()
        .get("https://api.bing.microsoft.com/v7.0/images/search")
        .query(&[("q", query)])
        .query(&[image_type])
        .query(&[("safeSearch", "strict")])
        .header("Ocp-Apim-Subscription-Key", key)
        .send()
        .await?
        .error_for_status()?
        .json::<ImagesResponse>()
        .await?;

    let res = WebImageSearchResponse {
        images: res
            .value
            .into_iter()
            .map(|it| WebImageSearchItem {
                thumbnail_url: it.thumbnail_url,
                url: it.content_url,
            })
            .collect(),
    };

    Ok(res)
}
