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
struct ImageTranslate {
    image_id: ImageId,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JigTranslate {
    jig_data_id: Uuid,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceTranslate {
    resource_data_id: Uuid,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CourseTranslate {
    course_data_id: Uuid,
    text: String,
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

        for count in 0..4 {
            let res = match count {
                0 => self
                    .update_image_translations()
                    .await
                    .context("update image translation task errored"),
                1 => self
                    .update_jig_translations()
                    .await
                    .context("update jig translation task errored"),
                2 => self
                    .update_course_translations()
                    .await
                    .context("update course translation task errored"),
                3 => self
                    .update_resource_translations()
                    .await
                    .context("update resource translation task errored"),

                _ => continue,
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
        log::info!("reached update images translation");
        let mut txn = self.db.begin().await?;

        let descriptions: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select id                               as "id!: ImageId",
       description
from image_metadata
     inner join image_upload on id = image_id
where description <> '' and translated_description = '{}'
and processed_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| ImageTranslate {
            image_id: row.id,
            text: row.description,
        })
        .try_collect()
        .await?;

        let names: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select id                               as "id!: ImageId",
       name
from image_metadata
     inner join image_upload on id = image_id
where name <> '' and translated_name = '{}'
and processed_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| ImageTranslate {
            image_id: row.id,
            text: row.name,
        })
        .try_collect()
        .await?;

        if descriptions.is_empty() && names.is_empty() {
            return Ok(true);
        }

        for t in descriptions {
            let descriptions: anyhow::Result<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await;

            match descriptions {
                Ok(descriptions) => {
                    if let Some(descriptions) = descriptions {
                        sqlx::query!(
                            r#"
                                update image_metadata
                                set translated_description = $2,
                                last_synced_at = null
                                where id = $1
                                "#,
                            t.image_id.0,
                            json!(descriptions)
                        )
                        .execute(&mut txn)
                        .await?;
                    } else {
                        log::warn!("Empty translation list for image_id: {}", t.image_id.0,);
                        continue;
                    };
                }
                Err(error) => {
                    log::error!(
                        "Could not translate image description. image_id: {}, error: {}",
                        t.image_id.0,
                        error,
                    );

                    continue;
                }
            }
        }

        for t in names {
            let names: anyhow::Result<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await;

            match names {
                Ok(names) => {
                    if let Some(names) = names {
                        sqlx::query!(
                            r#"
                                update image_metadata
                                set translated_name = $2,
                                last_synced_at = null
                                where id = $1
                                "#,
                            t.image_id.0,
                            json!(names)
                        )
                        .execute(&mut txn)
                        .await?;
                    } else {
                        log::warn!("Empty translation list for image_id: {}", t.image_id.0,);
                        continue;
                    };
                }
                Err(error) => {
                    log::error!(
                        "Could not translate image name. image_id: {}, error: {}",
                        t.image_id.0,
                        error,
                    );

                    continue;
                }
            }
        }

        txn.commit().await?;

        log::info!("completed update image translations");

        Ok(true)
    }

    async fn update_jig_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update JIG translation");
        let mut txn = self.db.begin().await?;

        let descriptions: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select jig_data.id,
       description
from jig_data
inner join jig on live_id = jig_data.id
where description <> '' and translated_description = '{}'
and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| JigTranslate {
            jig_data_id: row.id,
            text: row.description,
        })
        .try_collect()
        .await?;

        let display_names: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select jig_data.id,
       display_name
from jig_data
inner join jig on live_id = jig_data.id
where display_name <> '' and translated_name = '{}'
and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
         "#
        )
        .fetch(&mut txn)
        .map_ok(|row| JigTranslate {
            jig_data_id: row.id,
            text: row.display_name,
        })
        .try_collect()
        .await?;

        if descriptions.is_empty() && display_names.is_empty() {
            return Ok(true);
        }

        for t in descriptions {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update jig_data
                            set translated_description = $2,
                                last_synced_at = null
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
                    t.text
                );

                continue;
            };
        }

        for t in display_names {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update jig_data
                            set translated_name = $2,
                                last_synced_at = null
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
                    t.text
                );

                continue;
            };
        }

        txn.commit().await?;

        log::info!("completed update jig translations");

        Ok(true)
    }

    async fn update_resource_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update Resource translation");
        let mut txn = self.db.begin().await?;

        let descriptions: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select resource_data.id,
       description
from resource_data
inner join resource on live_id = resource_data.id
where description <> '' and translated_description = '{}'
and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 5 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| ResourceTranslate {
            resource_data_id: row.id,
            text: row.description,
        })
        .try_collect()
        .await?;

        let display_names: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select resource_data.id,
       display_name
from resource_data
inner join resource on live_id = resource_data.id
where display_name <> '' and translated_name = '{}'
and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 5 for no key update skip locked;
         "#
        )
        .fetch(&mut txn)
        .map_ok(|row| ResourceTranslate {
            resource_data_id: row.id,
            text: row.display_name,
        })
        .try_collect()
        .await?;

        if descriptions.is_empty() && display_names.is_empty() {
            return Ok(true);
        }

        for t in descriptions {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update resource_data
                            set translated_description = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.resource_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!(
                        "Empty translation list for resource_data_id: {}",
                        t.resource_data_id
                    );
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate resource_data_id: {}, string: {}",
                    t.resource_data_id,
                    t.text
                );

                continue;
            };
        }

        for t in display_names {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update resource_data
                            set translated_name = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.resource_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!(
                        "Empty translation list for resource_data_id: {}",
                        t.resource_data_id
                    );
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate resource_data_id: {}, string: {}",
                    t.resource_data_id,
                    t.text
                );

                continue;
            };
        }

        txn.commit().await?;

        log::info!("completed update resource translations");

        Ok(true)
    }

    async fn update_course_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update Course translation");
        let mut txn = self.db.begin().await?;

        let descriptions: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select course_data.id,
       description
from course_data
inner join course on live_id = course_data.id
where description <> ''
      and translated_description = '{}'
      and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| CourseTranslate {
            course_data_id: row.id,
            text: row.description,
        })
        .try_collect()
        .await?;

        let names: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select course_data.id,
       display_name
from course_data
inner join course on live_id = course_data.id
where display_name <> ''
      and translated_name = '{}'
      and published_at is not null
order by coalesce(updated_at, created_at) desc
limit 50 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| CourseTranslate {
            course_data_id: row.id,
            text: row.display_name,
        })
        .try_collect()
        .await?;

        if descriptions.is_empty() && names.is_empty() {
            return Ok(true);
        }

        for t in descriptions {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update course_data
                            set translated_description = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.course_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!("Empty translation list for course_id: {}", t.course_data_id);
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate course_id: {}, string: {}",
                    t.course_data_id,
                    t.text
                );

                continue;
            };
        }

        for t in names {
            let res: Option<Option<HashMap<String, String>>> =
                multi_translation(&t.text, &self.api_key).await.ok();

            if let Some(res) = res {
                if let Some(res) = res {
                    sqlx::query!(
                        r#"
                            update course_data
                            set translated_name = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.course_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;
                } else {
                    log::debug!("Empty translation list for course_id: {}", t.course_data_id);
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate course_id: {}, string: {}",
                    t.course_data_id,
                    t.text
                );

                continue;
            };
        }
        txn.commit().await?;

        log::info!("completed update Course translations");

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
        };
    }

    if translation_list.is_empty() {
        return Ok(None);
    }

    Ok(Some(translation_list))
}

// #[sqlx::test]
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
