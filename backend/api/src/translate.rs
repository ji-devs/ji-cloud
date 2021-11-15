use reqwest::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest {
    q: Vec<String>,
    target: String,
    source: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponse {
    data: TranslateTextResponseList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseList {
    translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translation {
    translated_text: String,
}

pub async fn translate_text(
    query: &str,
    target: &str,
    source: &str,
    api_key: &str,
) -> anyhow::Result<Option<String>> {
    let queries = vec![query.to_string()];

    //https://cloud.google.com/translate/docs/languages
    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2")
        .query(&[("key", &api_key.to_owned())])
        .json(&TranslateTextRequest {
            q: queries,
            target: target.to_string(),
            source: source.to_string(),
        })
        .send()
        .await?
        .error_for_status()?
        .json::<TranslateTextResponse>()
        .await?;

    let res = TranslateTextResponseList {
        translations: res
            .data
            .translations
            .into_iter()
            .map(|it| Translation {
                translated_text: it.translated_text,
            })
            .collect(),
    };

    let translate = res.translations[0].translated_text.clone();

    Ok(Some(translate))
}
