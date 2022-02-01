use anyhow::Context;
use core::settings::RuntimeSettings;
use futures::TryStreamExt;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;

use shared::domain::image::ImageId;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest {
    q: Vec<String>,
    target: String,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectedLanguageResponse {
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectLanguageRequest {
    pub q: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponse {
    data: TranslateTextResponseList,
}

// Response from google translate detection
////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectLanguageResponse {
    data: Detection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Detection {
    detections: Vec<Vec<DetectLanguage>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectLanguage {
    language: String,
}

////////////////////////////////////////////////////////////

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

const LANGUAGES: &'static [&str] = &[
    "en", "iw", "es", "pt", "ru", "fr", "nl", "sv", "ar", "de", "hu", "it", "yi",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImageTranslateDescriptions {
    image_id: ImageId,
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JigTranslateDescriptions {
    jig_data_id: Uuid,
    description: String,
}

#[derive(Clone)]
pub struct GoogleTranslate {
    pub db: PgPool,
    pub api_key: String,
}

impl GoogleTranslate {
    pub fn new(db: PgPool, runtime_settings: &RuntimeSettings) -> anyhow::Result<Option<Self>> {
        let api_key = match runtime_settings.google_api_key.to_owned() {
            Some(key) => key,
            None => return Ok(None),
        };

        Ok(Some(Self { db, api_key }))
    }

    pub async fn spawn_cron_jobs(&self) -> anyhow::Result<()> {
        log::debug!("reached description translation cron job");

        for count in 0..2 {
            let res = if count == 0 {
                self.update_jig_translations()
                    .await
                    .context("update jig description translation task errored")
            } else {
                self.update_image_translations()
                    .await
                    .context("update images description translation task errored")
            };

            match res {
                Ok(true) => {}
                Ok(false) => {
                    log::info!("exiting translation api");
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    sentry::integrations::anyhow::capture_anyhow(&e);
                }
            }
        }
        Ok(())
    }

    async fn update_image_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update images translation descriptions");
        let mut txn = self.db.begin().await?;

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select id                               as "id!: ImageId",
       description                                                                                    
from image_metadata
     join image_upload on id = image_id
where description <> '' and translated_description = '{}'
and processed_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| ImageTranslateDescriptions {
            image_id: row.id,
            description: row.description
        })
        .try_collect()
        .await?;

        if requests.is_empty() {
            return Ok(true);
        }

        log::debug!(
            "Updating a batch of {} image description(s)",
            requests.len()
        );

        for t in requests {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.description, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                        update image_metadata 
                        set translated_description = $2,
                            updated_at = now()
                        where id = $1
                        "#,
                        t.image_id.0,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!("Empty translation list for image_id: {}", t.image_id.0,);
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate image_id: {}, string: {}",
                    t.image_id.0,
                    t.description
                );

                continue;
            }
        }

        txn.commit().await?;

        log::info!("completed update image description translations");

        Ok(true)
    }

    async fn update_jig_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update JIG description translation");
        let mut txn = self.db.begin().await?;

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select jig_data.id,
       description                                                                                    
from jig_data
where description <> '' and translated_description = '{}'
and draft_or_live is not NULL
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| JigTranslateDescriptions {
            jig_data_id: row.id,
            description: row.description
        })
        .try_collect()
        .await?;

        if requests.is_empty() {
            return Ok(true);
        }

        log::debug!("Updating a batch of {} jig description(s)", requests.len());

        for t in requests {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.description, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update jig_data 
                            set translated_description = $2,
                                updated_at = now()
                            where id = $1
                            "#,
                        &t.jig_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!("Empty translation list for jig_data_id: {}", t.jig_data_id);
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate jig_data_id: {}, string: {}",
                    t.jig_data_id,
                    t.description
                );

                continue;
            };
        }

        txn.commit().await?;

        log::info!("completed update jig description translations");

        Ok(true)
    }
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

pub async fn multi_translation(
    description: &str,
    api_key: &str,
) -> anyhow::Result<Option<HashMap<String, String>>> {
    //https://cloud.google.com/translate/docs/languages
    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2/detect")
        .query(&[("key", &api_key.to_owned())])
        .json(&DetectLanguageRequest {
            q: description.to_string(),
        })
        .send()
        .await?
        .json::<DetectLanguageResponse>()
        .await?;

    let language_list: Vec<_> = res.data.detections.into_iter().collect();

    let v: Vec<String> = {
        let mut s: Vec<String> = Vec::new();
        for lang in language_list {
            for l in lang {
                s.push(l.language.to_string().to_owned());
            }
        }
        s
    };

    let src = &v[0];

    let mut translation_list = HashMap::new();

    for l in LANGUAGES {
        let text = if l != src {
            let text: Option<String> =
                translate_text(description, l, src, &api_key.to_owned()).await?;
            text
        } else {
            None
        };

        if let Some(meep) = text {
            translation_list.insert(l.to_owned().to_owned(), meep);
        } else {
            translation_list.insert(l.to_owned().to_owned(), description.to_owned());
        }
    }

    if translation_list.is_empty() {
        return Ok(None);
    }

    Ok(Some(translation_list))
}

// #[actix_rt::test]
// async fn test_translated_description() -> anyhow::Result<()> {
//     let api_key: &str = "";

//     let this_arr: Vec<&str> = vec!["yes", "...", "passed", "", "nothing", ":?", ",", "Good"];

//     for t in this_arr {
//         let translated_description: Option<Option<HashMap<String, String>>> =
//             multi_translation(t, api_key).await.ok();

//         if let Some(translation) = translated_description {
//             println!("result {:?}", translation);
//             translation
//         } else {
//             continue;
//         };
//     }

//     Ok(())
// }
