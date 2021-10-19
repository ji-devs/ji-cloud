use serde::Deserialize;
use shared::domain::search::{WebImageSearchItem, WebImageSearchResponse};

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct TranslateRequest {
//     q: String,
//     target: String,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponse {
    data: TranslateTextResponseList
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponseList {
    data: TranslateTextResponseList
}

pub async fn translate(query: &str, target: &str, key: &str) -> anyhow::Result<WebImageSearchResponse> {

    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2")
        .query(&[("q", query),("target", target)])
        .header("Authorization: Bearer", token)
        .send()
        .await?
        .error_for_status()?
        .json::<TranslateTextResponse>()
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
