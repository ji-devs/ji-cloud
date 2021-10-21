use crate::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest {
    q: Vec<String>,
    target: String,
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
    access_token: &str,
) -> Result<String, TranslateTextError> {
    let queries = vec![query.to_string()];

    //https://cloud.google.com/translate/docs/languages
    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token.to_owned()),
        )
        .header("Content-Type: application/json; charset={}", "utf-8")
        .json(&TranslateTextRequest {
            q: queries,
            target: target.to_string(),
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

    Ok(translate)
}

enum TranslateTextError {
    TranslateResponseError(reqwest::Error),
    Sqlx(sqlx::Error),
}

impl From<reqwest::Error> for TranslateTextError {
    fn from(err: reqwest::Error) -> Self {
        Self::TranslateResponseError(err)
    }
}

impl<T> From<T> for TranslateTextError {
    fn into(self) -> T {
        Self::Sqlx()
    }
}
