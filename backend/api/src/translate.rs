use anyhow::Context;
use core::settings::RuntimeSettings;
use futures::TryStreamExt;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgConnection, PgPool};
use std::collections::HashMap;

use shared::domain::image::ImageId;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextRequest {
    q: String,
    source: String,
    target: String,
    format: String,
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
struct PlaylistTranslate {
    playlist_data_id: Uuid,
    text: String,
}

/// Translation status for assets
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[serde(rename_all = "camelCase")]
#[repr(i8)]
pub enum TranslationStatus {
    /// Google Translation response for strings that can't detect a language
    Undefined = 0,

    /// API cannot translate language
    NoTranslation = 1,

    /// Successful Translation
    Success = 2,
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
                    .update_playlist_translations()
                    .await
                    .context("update playlist translation task errored"),
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
and description_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 10 for no key update skip locked;
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
and name_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 10 for no key update skip locked;
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
                        update_image_translation_status(
                            &mut txn,
                            "image_metadata",
                            "description",
                            t.image_id.0,
                            TranslationStatus::Success,
                        )
                        .await?;
                    } else {
                        log::warn!("Empty translation list for image_id: {}", t.image_id.0,);
                        update_image_translation_status(
                            &mut txn,
                            "image_metadata",
                            "description",
                            t.image_id.0,
                            TranslationStatus::Undefined,
                        )
                        .await?;
                        continue;
                    };
                }
                Err(error) => {
                    log::error!(
                        "Could not translate description for image_id: {}, error: {}",
                        t.image_id.0,
                        error,
                    );
                    update_image_translation_status(
                        &mut txn,
                        "image_metadata",
                        "description",
                        t.image_id.0,
                        TranslationStatus::NoTranslation,
                    )
                    .await?;

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
                        update_image_translation_status(
                            &mut txn,
                            "image_metadata",
                            "name",
                            t.image_id.0,
                            TranslationStatus::Success,
                        )
                        .await?;
                    } else {
                        log::warn!("Empty translation list for image_id: {}", t.image_id.0,);
                        update_image_translation_status(
                            &mut txn,
                            "image_metadata",
                            "name",
                            t.image_id.0,
                            TranslationStatus::Undefined,
                        )
                        .await?;
                        continue;
                    };
                }
                Err(error) => {
                    log::error!(
                        "Could not translate name for image_id: {}, error: {}",
                        t.image_id.0,
                        error,
                    );
                    update_image_translation_status(
                        &mut txn,
                        "image_metadata",
                        "name",
                        t.image_id.0,
                        TranslationStatus::NoTranslation,
                    )
                    .await?;

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
select jig_data.id ,
       description
from jig_data
inner join jig on live_id = jig_data.id
where description <> '' and translated_description = '{}'
and published_at is not null
and description_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 30 for no key update skip locked;
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
and name_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 30 for no key update skip locked;
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
                    update_asset_translation_status(
                        &mut txn,
                        "jig",
                        "description",
                        t.jig_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!("Empty translation list for jig_data_id: {}", t.jig_data_id);
                    update_asset_translation_status(
                        &mut txn,
                        "jig",
                        "description",
                        t.jig_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate description for jig_data_id: {}, string: {}",
                    t.jig_data_id,
                    t.text
                );
                update_asset_translation_status(
                    &mut txn,
                    "jig",
                    "description",
                    t.jig_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;

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
                    update_asset_translation_status(
                        &mut txn,
                        "jig",
                        "name",
                        t.jig_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!("Empty translation list for jig_data_id: {}", t.jig_data_id);
                    update_asset_translation_status(
                        &mut txn,
                        "jig",
                        "name",
                        t.jig_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;

                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate name for jig_data_id: {}, string: {}",
                    t.jig_data_id,
                    t.text
                );
                update_asset_translation_status(
                    &mut txn,
                    "jig",
                    "name",
                    t.jig_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;

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
and description_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 10 for no key update skip locked;
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
and name_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 10 for no key update skip locked;
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
                    update_asset_translation_status(
                        &mut txn,
                        "resource",
                        "description",
                        t.resource_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!(
                        "Empty description translation list for resource_data_id: {}",
                        t.resource_data_id
                    );
                    update_asset_translation_status(
                        &mut txn,
                        "resource",
                        "description",
                        t.resource_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate description for resource_data_id: {}, string: {}",
                    t.resource_data_id,
                    t.text
                );
                update_asset_translation_status(
                    &mut txn,
                    "resource",
                    "description",
                    t.resource_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;

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

                    update_asset_translation_status(
                        &mut txn,
                        "resource",
                        "name",
                        t.resource_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!(
                        "Empty name translation list for resource_data_id: {}",
                        t.resource_data_id
                    );
                    update_asset_translation_status(
                        &mut txn,
                        "resource",
                        "name",
                        t.resource_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;

                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate name for resource_data_id: {}, string: {}",
                    t.resource_data_id,
                    t.text
                );

                update_asset_translation_status(
                    &mut txn,
                    "resource",
                    "name",
                    t.resource_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;

                continue;
            };
        }

        txn.commit().await?;

        log::info!("completed update resource translations");

        Ok(true)
    }

    async fn update_playlist_translations(&self) -> anyhow::Result<bool> {
        log::info!("reached update Playlist translation");
        let mut txn = self.db.begin().await?;

        let descriptions: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select playlist_data.id,
       description
from playlist_data
inner join playlist on live_id = playlist_data.id
where description <> ''
      and translated_description = '{}'
      and published_at is not null
      and description_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 20 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| PlaylistTranslate {
            playlist_data_id: row.id,
            text: row.description,
        })
        .try_collect()
        .await?;

        let names: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select playlist_data.id,
       display_name
from playlist_data
inner join playlist on live_id = playlist_data.id
where display_name <> ''
      and translated_name = '{}'
      and published_at is not null
      and name_translate_status is null
order by coalesce(updated_at, created_at) desc
limit 20 for no key update skip locked;
 "#
        )
        .fetch(&mut txn)
        .map_ok(|row| PlaylistTranslate {
            playlist_data_id: row.id,
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
                            update playlist_data
                            set translated_description = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.playlist_data_id,
                        json!(res),
                    )
                    .execute(&mut txn)
                    .await?;

                    update_asset_translation_status(
                        &mut txn,
                        "playlist",
                        "description",
                        t.playlist_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!("Empty translation list for playlist_id: {}", t.playlist_data_id);
                    update_asset_translation_status(
                        &mut txn,
                        "playlist",
                        "description",
                        t.playlist_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;

                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate description for playlist_id: {}, string: {}",
                    t.playlist_data_id,
                    t.text
                );

                update_asset_translation_status(
                    &mut txn,
                    "playlist",
                    "description",
                    t.playlist_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;

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
                            update playlist_data
                            set translated_name = $2,
                                last_synced_at = null
                            where id = $1
                            "#,
                        &t.playlist_data_id,
                        json!(res)
                    )
                    .execute(&mut txn)
                    .await?;

                    update_asset_translation_status(
                        &mut txn,
                        "playlist",
                        "name",
                        t.playlist_data_id,
                        TranslationStatus::Success,
                    )
                    .await?;
                } else {
                    log::debug!("Empty translation list for playlist_id: {}", t.playlist_data_id);
                    update_asset_translation_status(
                        &mut txn,
                        "playlist",
                        "name",
                        t.playlist_data_id,
                        TranslationStatus::Undefined,
                    )
                    .await?;
                    continue;
                };
            } else {
                log::debug!(
                    "Could not translate name for playlist_id: {}, string: {}",
                    t.playlist_data_id,
                    t.text
                );

                update_asset_translation_status(
                    &mut txn,
                    "playlist",
                    "name",
                    t.playlist_data_id,
                    TranslationStatus::NoTranslation,
                )
                .await?;
                continue;
            };
        }
        txn.commit().await?;

        log::info!("completed update Playlist translations");

        Ok(true)
    }
}

pub async fn translate_text(
    query: &str,
    target: &str,
    source: &str,
    api_key: &str,
) -> anyhow::Result<Option<String>> {
    //https://cloud.google.com/translate/docs/languages
    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2")
        .query(&[("key", &api_key.to_owned())])
        .json(&TranslateTextRequest {
            q: query.to_string(),
            target: target.to_string(),
            source: source.to_string(),
            format: "text".to_string(),
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
    query: &str,
    api_key: &str,
) -> anyhow::Result<Option<HashMap<String, String>>> {
    //https://cloud.google.com/translate/docs/languages
    //https://cloud.google.com/translate/docs/reference/rest/v2/translate
    let res = reqwest::Client::new()
        .post("https://translation.googleapis.com/language/translate/v2/detect")
        .query(&[("key", &api_key.to_owned())])
        .json(&DetectLanguageRequest {
            q: query.to_string(),
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

    if &v[0] == "und" {
        return Ok(None);
    }

    let src = &v[0][..2];

    let mut translation_list = HashMap::new();

    for l in LANGUAGES {
        let text = if l != &src {
            let text: Option<String> = translate_text(query, l, src, &api_key.to_owned()).await?;
            text
        } else {
            None
        };

        if let Some(text) = text {
            translation_list.insert(l.to_owned().to_owned(), text);
        } else {
            translation_list.insert(l.to_owned().to_owned(), query.to_owned());
        };
    }

    Ok(Some(translation_list))
}

async fn update_asset_translation_status(
    conn: &mut PgConnection,
    table: &str,
    field: &str,
    id: Uuid,
    status: TranslationStatus,
) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "update {0} set {1}_translate_status = $2 where {0}.live_id = $1",
        table, field
    ))
    .bind(id)
    .bind(status as i16)
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn update_image_translation_status(
    conn: &mut PgConnection,
    table: &str,
    field: &str,
    id: Uuid,
    status: TranslationStatus,
) -> sqlx::Result<()> {
    sqlx::query(&format!(
        "update {0} set {1}_translate_status = $2 where {0}.id = $1",
        table, field
    ))
    .bind(id)
    .bind(status as i16)
    .execute(&mut *conn)
    .await?;

    Ok(())
}

// #[test_service(setup = "setup_service", fixtures(""))]
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
