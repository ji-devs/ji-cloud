use algolia::{
    filter::{AndFilterable, BooleanFilter, CmpFilter, CommonFilter, FacetFilter, FilterOperator},
    request::{BatchWriteRequests, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
use anyhow::Context;
use chrono::Utc;
use core::settings::AlgoliaSettings;
use futures::TryStreamExt;
use serde::Serialize;
use shared::domain::{
    category::CategoryId, image::ImageId, meta::AffiliationId, meta::AgeRangeId, meta::StyleId,
};
use sqlx::PgPool;
use std::{convert::TryInto, time::Duration, time::Instant};
use tokio::task::JoinHandle;
use uuid::Uuid;

use migration::ResyncKind;

mod migration;

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
    publish_at: Option<i64>,
    is_premium: bool,
}

pub struct Updater {
    pub db: PgPool,
    pub algolia_client: AlgoliaClient,
}

impl Updater {
    pub fn spawn(self) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            if self.algolia_client.inner.is_none() {
                return;
            }

            loop {
                let iteration_start = Instant::now();

                if let Err(e) = self
                    .update_images()
                    .await
                    .context("update images task errored")
                {
                    log::error!("{:?}", e);
                    sentry::integrations::anyhow::capture_anyhow(&e);
                }

                tokio::time::delay_until((iteration_start + Duration::from_secs(5)).into()).await;
            }
        })
    }

    async fn update_images(&self) -> anyhow::Result<()> {
        let sync_time = Utc::now();

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            r#"
select id,
    name,
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
    publish_at,
    is_premium
 from image_metadata
 where last_synced_at is null or (updated_at is not null and last_synced_at < updated_at and updated_at <= $1)
 limit 100;
     "#, &sync_time
        )
        .fetch(&self.db)
        .map_ok(|row| algolia::request::BatchWriteRequest::UpdateObject {
            body: match serde_json::to_value(&BatchImage {
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
                publish_at: row.publish_at.map(|t| t.timestamp_nanos()),
                is_premium: row.is_premium,
            })
            .expect("failed to serialize BatchImage to json")
            {
                serde_json::Value::Object(map) => map,
                _ => panic!("failed to serialize BatchImage to json map"),
            },
            object_id: row.id.to_string(),
        })
        .try_collect()
        .await?;

        if requests.is_empty() {
            return Ok(());
        }

        log::debug!("Updating a batch of {} image(s)", requests.len());

        let request = algolia::request::BatchWriteRequests { requests };
        let ids = self.algolia_client.batch_images(request).await?;

        log::debug!("Updated a batch of {} image(s)", ids.len());

        sqlx::query!(
            "update image_metadata set last_synced_at = $1 where id = any($2)",
            sync_time,
            &ids
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}

macro_rules! with_client {
    ($client:expr; $ret:expr) => {{
        let c: &Inner = match &$client {
            Some(c) => c,
            None => return Ok($ret),
        };

        c
    }};

    ($client:expr) => { with_client!($client; ()) };
}

#[derive(Clone)]
pub struct AlgoliaClient {
    inner: Option<Inner>,
    index: String,
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

impl AlgoliaClient {
    pub async fn migrate(&self, pool: &PgPool) -> anyhow::Result<()> {
        // We can't exactly access algolia if we don't have a client.
        let inner = with_client!(self.inner; ());

        let mut txn = pool.begin().await?;

        let algolia_version = sqlx::query!(
            r#"
with new_row as (
    insert into "settings" (algolia_index_name) values($1) on conflict(singleton) do nothing returning algolia_index_version    
)
select algolia_index_version as "algolia_index_version!" from new_row
union
select algolia_index_version as "algolia_index_version!" from "settings" where algolia_index_name = $1
"#, &self.index,
        )
        .fetch_optional(&mut txn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("algolia index mismatch (error is to avoid messing up algolia indexes by using already existing dbs, unfortunately no checking can be done to ensure that it works the other way around.)"))?
        .algolia_index_version;

        if algolia_version == migration::INDEX_VERSION {
            return Ok(());
        }

        let migrations_to_run = &migration::INDEXING_MIGRATIONS[(algolia_version as usize)..];

        for (idx, (_, updater)) in migrations_to_run.iter().enumerate() {
            updater(inner, &self.index).await.with_context(|| {
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

    async fn batch_images(&self, batch: BatchWriteRequests) -> anyhow::Result<Vec<Uuid>> {
        let resp = with_client!(self.inner; vec![])
            .batch(&self.index, &batch)
            .await?;

        let ids: Result<Vec<_>, _> = resp
            .object_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id))
            .collect();

        eprintln!("{:?}", ids.as_ref().ok().map(|it| it.len()));

        Ok(dbg!(ids?))
    }

    pub fn new(settings: Option<AlgoliaSettings>) -> anyhow::Result<Self> {
        if let Some(settings) = settings {
            let app_id = algolia::AppId::new(settings.application_id);
            let api_key = algolia::ApiKey(settings.key);

            Ok(Self {
                inner: Some(Inner::new(app_id, api_key)?),
                index: settings.index,
            })
        } else {
            Ok(Self {
                inner: None,
                index: String::new(),
            })
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
    ) -> anyhow::Result<(Vec<Uuid>, u32, u64)> {
        let compare_time = Utc::now().timestamp_nanos();

        let client = with_client!(self.inner; (vec![], 0, 0));

        let mut filters = algolia::filter::AndFilter { filters: vec![] };

        if let Some(is_published) = is_published {
            filters.filters.push(Box::new(CommonFilter {
                filter: CmpFilter::new("publish_at".to_owned(), FilterOperator::Le, compare_time),
                invert: !is_published,
            }))
        }

        if let Some(is_premium) = is_premium {
            filters.filters.push(Box::new(CommonFilter {
                filter: BooleanFilter {
                    facet_name: "is_premium".to_owned(),
                    value: is_premium,
                },
                invert: false,
            }))
        }

        filters_for_ids(&mut filters.filters, "styles", styles);
        filters_for_ids(&mut filters.filters, "age_ranges", age_ranges);
        filters_for_ids(&mut filters.filters, "affiliations", affiliations);
        filters_for_ids(&mut filters.filters, "categories", categories);

        let results: SearchResponse = client
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

        Ok((results, pages, total_hits))
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
        with_client!(self.inner)
            .delete_object(&self.index, &id.to_string())
            .await?;

        Ok(())
    }
}
