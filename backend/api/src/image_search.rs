use serde::Deserialize;
use shared::domain::search::{WebImageSearchItem, WebImageSearchResponse};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Image {
    thumbnail_url: String,
    content_url: String,
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

pub async fn get_images(query: &str, key: &str) -> anyhow::Result<WebImageSearchResponse> {
    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/endpoints
    // https://docs.microsoft.com/en-us/bing/search-apis/bing-image-search/reference/query-parameters
    let res = reqwest::Client::new()
        .get("https://api.bing.microsoft.com/v7.0/images/search")
        .query(&[("q", query)])
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
