use algolia::{
    filter::{AndFilterable, BooleanFilter, CmpFilter, CommonFilter, FacetFilter, FilterOperator},
    model::attribute::Attribute,
    model::attribute::SearchableAttributes,
    request::SetSettings,
    request::{BatchWriteRequests, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
use anyhow::Context;
use chrono::Utc;
use core::settings::AlgoliaSettings;
use futures::{future::BoxFuture, TryStreamExt};
use serde::Serialize;
use shared::domain::{
    category::CategoryId, image::ImageId, meta::AffiliationId, meta::AgeRangeId, meta::StyleId,
};
use sqlx::PgPool;
use std::{convert::TryInto, time::Duration, time::Instant};
use tokio::task::JoinHandle;
use uuid::Uuid;
#[derive(Serialize)]
struct BatchImage<'a> {
    name: &'a str,
    description: &'a str,
    styles: &'a [Uuid],
    age_ranges: &'a [Uuid],
    affiliations: &'a [Uuid],
    categories: &'a [Uuid],
    category_names: &'a [String],
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

                if let Err(e) = self.update_images().await {
                    log::error!("update images task failed {} (retrying after pause)", e);
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
select 
    id,
    name,
    description,
    array((select affiliation_id from image_affiliation where image_id = image_metadata.id)) as "affiliations!",
    array((select style_id from image_style where image_id = image_metadata.id)) as "styles!",
    array((select age_range_id from image_age_range where image_id = image_metadata.id)) as "age_ranges!",
    array((select category_id from image_category where image_id = image_metadata.id)) as "categories!",
    array((select name from category inner join image_category on category_id = image_category.category_id where image_category.image_id = image_metadata.id)) as "category_names!"
from image_metadata
where last_synced_at is null or (updated_at is not null and last_synced_at < updated_at and updated_at <= $1)
limit 100
"#, &sync_time
        )
        .fetch(&self.db)
        .map_ok(|row| algolia::request::BatchWriteRequest::UpdateObject {
            body: match serde_json::to_value(&BatchImage {
                name: &row.name,
                description: &row.description,
                styles: &row.styles,
                age_ranges: &row.age_ranges,
                affiliations: &row.affiliations,
                categories: &row.categories,
                category_names: &row.category_names
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

        let request = algolia::request::BatchWriteRequests { requests };
        let ids = self.algolia_client.batch_images(request).await?;

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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub styles: &'a [StyleId],
    pub age_ranges: &'a [AgeRangeId],
    pub affiliations: &'a [AffiliationId],
    pub categories: &'a [CategoryId],
    pub category_names: &'a [String],
}

#[derive(Serialize, Default, Debug)]
pub struct ImageUpdate<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<&'a [StyleId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_ranges: Option<&'a [AgeRangeId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliations: Option<&'a [AffiliationId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<&'a [CategoryId]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_names: Option<&'a [String]>,
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

#[derive(Copy, Clone, Debug)]
enum ResyncKind {
    None,
    Complete,
}

fn algolia_bad_batch_object<'a>(
    client: &'a Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    Box::pin(async move {
        client.delete_object(index, "batch").await?;
        Ok(())
    })
}

fn algolia_set_searchable_fields<'a>(
    client: &'a Inner,
    index: &'a str,
) -> BoxFuture<'a, anyhow::Result<()>> {
    Box::pin(async move {
        client
            .set_settings(
                index,
                &SetSettings {
                    searchable_attributes: Some(
                        SearchableAttributes::build()
                            .single(Attribute("name".to_owned()))
                            .single(Attribute("description".to_owned()))
                            .single(Attribute("category_names".to_owned()))
                            .finish(),
                    ),
                },
            )
            .await?;
        Ok(())
    })
}

const ALGOLIA_INDEXING_MIGRATIONS: &'static [(
    ResyncKind,
    for<'a> fn(&'a Inner, &'a str) -> BoxFuture<'a, anyhow::Result<()>>,
)] = &[
    (ResyncKind::Complete, algolia_bad_batch_object),
    (ResyncKind::Complete, algolia_set_searchable_fields),
];

const ALGOLIA_INDEXING_VERSION: i16 = ALGOLIA_INDEXING_MIGRATIONS.len() as i16;

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

        if algolia_version == ALGOLIA_INDEXING_VERSION {
            return Ok(());
        }

        let migrations_to_run = &ALGOLIA_INDEXING_MIGRATIONS[(algolia_version as usize)..];

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
            ALGOLIA_INDEXING_VERSION
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

        let ids: Result<_, _> = resp
            .object_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id))
            .collect();

        Ok(ids?)
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
                filter: CmpFilter::new("publishAt".to_owned(), FilterOperator::Le, compare_time),
                invert: !is_published,
            }))
        }

        if let Some(is_premium) = is_premium {
            filters.filters.push(Box::new(CommonFilter {
                filter: BooleanFilter {
                    facet_name: "isPremium".to_owned(),
                    value: is_premium,
                },
                invert: false,
            }))
        }

        filters_for_ids(&mut filters.filters, "styles", styles);
        filters_for_ids(&mut filters.filters, "ageRanges", age_ranges);
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
