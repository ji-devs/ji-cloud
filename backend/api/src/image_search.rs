use serde::Deserialize;
use shared::domain::search::{WebImageSearchItem, WebImageSearchResponse};

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
    image_type: Option<String>,
    key: &str,
) -> anyhow::Result<WebImageSearchResponse> {
    let image_type = if let Some(image_type) = image_type {
        image_type
    } else {
        Default::default()
    };
    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/endpoints
    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/query-parameters
    let res = reqwest::Client::new()
        .get("https://api.bing.microsoft.com/v7.0/images/search")
        .query(&[("q", query), ("imageType", &image_type)])
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
