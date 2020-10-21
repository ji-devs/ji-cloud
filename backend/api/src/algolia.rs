use algolia::{
    filter::{AndFilterable, BooleanFilter, CmpFilter, CommonFilter, FacetFilter, FilterOperator},
    request::{BatchWriteRequests, SearchQuery},
    response::SearchResponse,
    Client as Inner,
};
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

#[derive(Serialize)]
struct BatchImage<'a> {
    name: &'a str,
    description: &'a str,
    styles: &'a [Uuid],
    age_ranges: &'a [Uuid],
    affiliations: &'a [Uuid],
    categories: &'a [Uuid],
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
    array((select category_id from image_category where image_id = image_metadata.id)) as "categories!"
from image_metadata
where last_synced_at < updated_at is not false and updated_at <= $1 is not false
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

        if requests.len() == 0 {
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
    ) -> anyhow::Result<(Vec<Uuid>, u32)> {
        let compare_time = Utc::now().timestamp_nanos();

        let client = with_client!(self.inner; (vec![], 0));

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

        let results = results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok((results, pages))
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
