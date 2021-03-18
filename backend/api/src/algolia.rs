use algolia::{
    filter::{AndFilterable, CommonFilter, FacetFilter, TagFilter},
    request::{BatchWriteRequests, SearchQuery, VirtualKeyRestrictions},
    response::SearchResponse,
    ApiKey, AppId, Client as Inner,
};
use anyhow::Context;
use chrono::Utc;
use core::settings::AlgoliaSettings;
use futures::TryStreamExt;
use serde::Serialize;
use shared::{
    domain::{
        category::CategoryId,
        image::{ImageId, ImageKind},
        meta::AffiliationId,
        meta::AgeRangeId,
        meta::StyleId,
    },
    media::MediaGroupKind,
};
use sqlx::PgPool;
use std::{convert::TryInto, time::Duration, time::Instant};
use tokio::task::JoinHandle;
use uuid::Uuid;

use migration::ResyncKind;

mod migration;

const PREMIUM_TAG: &'static str = "premium";
const PUBLISHED_TAG: &'static str = "published";

#[derive(Serialize)]
struct BatchImage<'a> {
    name: &'a str,
    description: &'a str,
    styles: &'a [Uuid],
    style_names: &'a [String],
    age_ranges: &'a [Uuid],
    age_range_names: &'a [String],
    affiliations: &'a [Uuid],
    affiliation_names: &'a [String],
    categories: &'a [Uuid],
    category_names: &'a [String],
    media_subkind: &'a str,
    #[serde(rename = "_tags")]
    tags: Vec<&'static str>,
}

#[derive(Serialize)]
#[serde(tag = "media_kind")]
#[serde(rename_all = "camelCase")]
enum BatchMedia<'a> {
    Image(BatchImage<'a>),
}

pub struct Manager {
    pub db: PgPool,
    pub inner: Inner,
    pub index: String,
}

impl Manager {
    pub fn new(settings: Option<AlgoliaSettings>, db: PgPool) -> anyhow::Result<Option<Self>> {
        let (app_id, key, index) = match settings {
            Some(settings) => match (settings.management_key, settings.media_index) {
                (Some(key), Some(index)) => (settings.application_id, key, index),
                _ => return Ok(None),
            },
            None => return Ok(None),
        };

        Ok(Some(Self {
            inner: Inner::new(AppId::new(app_id), ApiKey(key))?,
            index,
            db,
        }))
    }

    #[must_use]
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            loop {
                let iteration_start = Instant::now();

                let res = self
                    .update_images()
                    .await
                    .context("update images task errored");

                match res {
                    Ok(true) => {}
                    Ok(false) => {
                        log::info!("exiting algolia indexing task (out of date)");
                        return;
                    }

                    Err(e) => {
                        log::error!("{:?}", e);
                        sentry::integrations::anyhow::capture_anyhow(&e);
                    }
                }

                tokio::time::delay_until((iteration_start + Duration::from_secs(5)).into()).await;
            }
        })
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        let mut txn = self.db.begin().await?;

        let algolia_version = sqlx::query!(
            r#"
with new_row as (
    insert into "settings" default values on conflict(singleton) do nothing returning algolia_index_version    
)
select algolia_index_version as "algolia_index_version!" from new_row
union
select algolia_index_version as "algolia_index_version!" from "settings"
"#,
        )
        .fetch_one(&mut txn)
        .await?
        .algolia_index_version;

        if algolia_version == migration::INDEX_VERSION {
            return Ok(());
        }

        let migrations_to_run = &migration::INDEXING_MIGRATIONS[(algolia_version as usize)..];

        for (idx, (_, updater)) in migrations_to_run.iter().enumerate() {
            updater(&self.inner, &self.index).await.with_context(|| {
                anyhow::anyhow!(
                    "error while running algolia updater #{}",
                    idx + (algolia_version as usize) + 1
                )
            })?;
        }

        // currently this can only be "no resync" or "complete resync" but eventually
        // we might want to be able to "resync everything that's only had an initial sync" or "everything that has had an update"
        let resync_mask =
            migrations_to_run
                .iter()
                .fold(ResyncKind::None, |acc, &(curr, _)| match (acc, curr) {
                    (_, ResyncKind::Complete) | (ResyncKind::Complete, _) => ResyncKind::Complete,
                    _ => ResyncKind::None,
                });

        match resync_mask {
            ResyncKind::Complete => {
                sqlx::query!("update image_metadata set last_synced_at = null")
                    .execute(&mut txn)
                    .await?;
            }
            ResyncKind::None => {}
        }

        sqlx::query!(
            r#"update "settings" set algolia_index_version = $1"#,
            migration::INDEX_VERSION
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    async fn batch_media(&self, batch: BatchWriteRequests) -> anyhow::Result<Vec<Uuid>> {
        let resp = self.inner.batch(&self.index, &batch).await?;

        let ids: Result<Vec<_>, _> = resp
            .object_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id))
            .collect();

        Ok(ids?)
    }

    async fn update_images(&self) -> anyhow::Result<bool> {
        let mut txn = self.db.begin().await?;

        let is_outdated = sqlx::query!(
            r#"select algolia_index_version != $1 as "outdated!" from settings"#,
            migration::INDEX_VERSION
        )
        .fetch_one(&mut txn)
        .await?
        .outdated;

        if is_outdated {
            return Ok(false);
        }

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            r#"
select id,
    name,
    kind as "kind: ImageKind",
    description,
    array((select affiliation_id from image_affiliation where image_id = image_metadata.id)) as "affiliations!",
    array((select affiliation.display_name
           from affiliation
                    inner join image_affiliation on affiliation.id = image_affiliation.affiliation_id
           where image_affiliation.image_id = image_metadata.id))                            as "affiliation_names!",
    array((select style_id from image_style where image_id = image_metadata.id))             as "styles!",
    array((select style.display_name
           from style
                    inner join image_style on style.id = image_style.style_id
           where image_style.image_id = image_metadata.id))                                  as "style_names!",
    array((select age_range_id from image_age_range where image_id = image_metadata.id))     as "age_ranges!",
    array((select age_range.display_name
           from age_range
                    inner join image_age_range on age_range.id = image_age_range.age_range_id
           where image_age_range.image_id = image_metadata.id))                              as "age_range_names!",
    array((select category_id from image_category where image_id = image_metadata.id))       as "categories!",
    array((select name
           from category
                    inner join image_category on category.id = image_category.category_id
           where image_category.image_id = image_metadata.id))                               as "category_names!",
    (publish_at < now() is true) as "is_published!",
    is_premium
from image_metadata
where 
    last_synced_at is null or
    (updated_at is not null and last_synced_at < updated_at) or
    (publish_at < now() is true and last_synced_at < publish_at)
limit 100
for no key update skip locked;
     "#
        )
        .fetch(&mut txn)
        .map_ok(|row| {
            let mut tags = Vec::new();
            if row.is_published {
                tags.push(PUBLISHED_TAG);
            }

            if row.is_premium {
                tags.push(PREMIUM_TAG);
            }

            algolia::request::BatchWriteRequest::UpdateObject {
            body: match serde_json::to_value(&BatchMedia::Image(BatchImage {
                media_subkind: &row.kind.to_str(),
                name: &row.name,
                description: &row.description,
                styles: &row.styles,
                style_names: &row.style_names,
                age_ranges: &row.age_ranges,
                age_range_names: &row.age_range_names,
                affiliations: &row.affiliations,
                affiliation_names: &row.affiliation_names,
                categories: &row.categories,
                category_names: &row.category_names,
                tags
            }))
            .expect("failed to serialize BatchImage to json")
            {
                serde_json::Value::Object(map) => map,
                _ => panic!("failed to serialize BatchImage to json map"),
            },
            object_id: row.id.to_string(),
        }})
        .try_collect()
        .await?;

        if requests.is_empty() {
            return Ok(true);
        }

        log::debug!("Updating a batch of {} image(s)", requests.len());

        let request = algolia::request::BatchWriteRequests { requests };
        let ids = self.batch_media(request).await?;

        log::debug!("Updated a batch of {} image(s)", ids.len());

        sqlx::query!(
            "update image_metadata set last_synced_at = now() where id = any($1)",
            &ids
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(true)
    }
}

#[derive(Clone)]
pub struct SearchKeyStore {
    frontend_search_parent_key: ApiKey,
}

impl SearchKeyStore {
    pub fn new(frontend_search_parent_key: String) -> anyhow::Result<Self> {
        Ok(Self {
            frontend_search_parent_key: ApiKey(frontend_search_parent_key),
        })
    }

    pub fn generate_virtual_key(
        &self,
        user_id: Option<Uuid>,
        ttl: Option<chrono::Duration>,
    ) -> ApiKey {
        self.frontend_search_parent_key
            .generate_virtual_key(&VirtualKeyRestrictions {
                user_token: user_id.map(|u| u.to_string()),
                valid_until: ttl.map(|ttl| Utc::now() + ttl),
            })
    }
}

fn filters_for_ids<T: Into<Uuid> + Copy>(
    filters: &mut Vec<Box<dyn AndFilterable>>,
    facet_name: &str,
    ids: &[T],
) {
    for id in ids.iter().copied() {
        let id: Uuid = id.into();
        filters.push(Box::new(CommonFilter {
            filter: FacetFilter {
                facet_name: facet_name.to_owned(),
                value: id.to_string(),
            },
            invert: false,
        }))
    }
}

fn media_filter(kind: MediaGroupKind, invert: bool) -> CommonFilter<FacetFilter> {
    CommonFilter {
        filter: FacetFilter {
            facet_name: "media_kind".to_owned(),
            value: kind.to_str().to_owned(),
        },
        invert,
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Inner,
    index: String,
}

impl Client {
    pub fn new(settings: Option<AlgoliaSettings>) -> anyhow::Result<Option<Self>> {
        if let Some(settings) = settings {
            let app_id = algolia::AppId::new(settings.application_id);

            let (inner, index) = match (settings.backend_search_key, settings.media_index) {
                (Some(key), Some(index)) => (Inner::new(app_id, ApiKey(key))?, index),
                _ => return Ok(None),
            };

            Ok(Some(Self { inner, index }))
        } else {
            Ok(None)
        }
    }

    // todo: return ImageId (can't because of repr issues in sqlx)
    pub async fn search_image(
        &self,
        query: &str,
        page: Option<u32>,
        is_premium: Option<bool>,
        is_published: Option<bool>,
        styles: &[StyleId],
        age_ranges: &[AgeRangeId],
        affiliations: &[AffiliationId],
        categories: &[CategoryId],
    ) -> anyhow::Result<Option<(Vec<Uuid>, u32, u64)>> {
        let mut filters = algolia::filter::AndFilter {
            filters: vec![Box::new(media_filter(MediaGroupKind::Image, false))],
        };

        if let Some(is_published) = is_published {
            filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(PUBLISHED_TAG.to_owned()),
                invert: !is_published,
            }))
        }

        if let Some(is_premium) = is_premium {
            filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(PREMIUM_TAG.to_owned()),
                invert: !is_premium,
            }))
        }

        filters_for_ids(&mut filters.filters, "styles", styles);
        filters_for_ids(&mut filters.filters, "age_ranges", age_ranges);
        filters_for_ids(&mut filters.filters, "affiliations", affiliations);
        filters_for_ids(&mut filters.filters, "categories", categories);

        let results: SearchResponse = self
            .inner
            .search(
                &self.index,
                SearchQuery {
                    query: Some(query),
                    page,
                    get_ranking_info: true,
                    filters: Some(filters),
                    hits_per_page: None,
                },
            )
            .await?;

        let pages = results.page_count.try_into()?;
        let total_hits = results.hit_count as u64;

        let results = results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some((results, pages, total_hits)))
    }

    pub async fn delete_image(&self, id: ImageId) {
        if let Err(e) = self.try_delete_image(id).await {
            log::warn!(
                "failed to delete image with id {} from algolia: {}",
                id.0.to_hyphenated(),
                e
            );
        }
    }

    pub async fn try_delete_image(&self, ImageId(id): ImageId) -> anyhow::Result<()> {
        self.inner
            .delete_object(&self.index, &id.to_string())
            .await?;

        Ok(())
    }
}
